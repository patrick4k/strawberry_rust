use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::mem::take;
use std::ops::Deref;
use std::rc::Rc;
use fxhash::FxHashMap;
use regex::{Captures, Regex};
use crate::grammar::grammar::{Grammar, LexerRule, ParserRule, Rule, RuleWrapper};
use serde::Deserialize;
use crate::grammar::grammar::RuleWrapper::{Multiton, Singleton};
use crate::util::util::OneOrMore;

#[derive(Deserialize)]
struct Data {
    lexer_rules: Vec<LexerData>,
    parser_rules: Vec<ParserData>
}

#[derive(Clone, Deserialize)]
struct LexerData {
    name: String,
    method: String,
    pattern: String
}

#[derive(Deserialize)]
struct ParserData {
    name: String,
    pattern: Option<String>,
    patterns: Option<Vec<String>>
}

impl Data {
    pub fn compute_lexer_rules(&self) -> Vec<LexerRule> {
        let mut lexer_rules = vec![];
        for data in &self.lexer_rules {
            let mut cap_group: usize = 0;
            let name = data.name.clone();
            let pattern = data.pattern.clone();
            let method = data.method.clone();
            let rule = match &*method {
                "Match" => LexerRule::Match { name, pattern },
                "RegexMatch" => LexerRule::RegexMatch { name, pattern: resolve_lexer_regex(pattern, &lexer_rules) },
                "Ignore" => LexerRule::Ignore { name, pattern: resolve_lexer_regex(pattern, &lexer_rules) },
                _ => {
                    let captures = Regex::new(r"Capture\((\d+)\)").unwrap().captures(&data.method);
                    let rule = match captures {
                        Some(captures) => {
                            cap_group = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let re = resolve_lexer_regex(data.pattern.clone(), &lexer_rules);
                            LexerRule::Capture { name: data.name.clone(), pattern: re, capture: cap_group }
                        }
                        None => {
                            panic!("Invalid method: {}", method);
                        }
                    };
                    rule
                }
            };
            lexer_rules.push(rule);
        }
        lexer_rules.reverse();
        lexer_rules
    }

    pub fn compute_parser_rules(&self, lexer_rules: Vec<LexerRule>) -> Vec<ParserRule> {
        let mut parser_rules: Vec<ParserRule> = vec![];
        let delimiter = Regex::new(r"\.").unwrap();
        for data in &self.parser_rules {
            let name = data.name.clone();
            let patterns = data.patterns.clone();
            let mut childrens = vec![];

            if data.pattern.is_some() {
                let pattern = data.pattern.clone().unwrap();
                let mut children = vec![];
                for child in delimiter.split(&pattern) {
                    children.push(resolve_parser_token(child, &lexer_rules, &parser_rules));
                }
                childrens.push(children);
            }

            for seq in patterns {
                for pattern in seq {
                    let mut children = vec![];
                    for child in delimiter.split(&pattern) {
                        children.push(resolve_parser_token(child, &lexer_rules, &parser_rules));
                    }
                    childrens.push(children);
                }
            }
            let sequence = OneOrMore::from(childrens);
            let rule = ParserRule::Recursive { name, sequence };
            parser_rules.push(rule);
        }
        parser_rules
    }
}

fn resolve_parser_token(token: &str, lexer_rules: &Vec<LexerRule>, parser_rules: &Vec<ParserRule>) -> RuleWrapper {
    let identifier = Regex::new(r"([a-zA-Z0-9_]+)(\W)").unwrap();
    let whitespace = Regex::new(r"\s+").unwrap();
    let token = whitespace.replace_all(token, "").to_string();
    let multiplier = identifier.captures(&*token);
    match multiplier {
        Some(cap) => {
            let token = cap.get(1).unwrap().as_str().to_string();
            match cap.get(2).unwrap().as_str() {
                "+" => Multiton {
                    rule: resolve_token(token, lexer_rules, parser_rules),
                    min: 1,
                    max: None
                },
                "*" => Multiton {
                    rule: resolve_token(token, lexer_rules, parser_rules),
                    min: 0,
                    max: None
                },
                "?" => Multiton {
                    rule: resolve_token(token, lexer_rules, parser_rules),
                    min: 0,
                    max: Some(1)
                },
                _ => panic!("Invalid multiplier: {}", cap.get(1).unwrap().as_str())
            }
        }
        None => Singleton(resolve_token(token, lexer_rules, parser_rules))
    }
}

fn resolve_token(token: String, lexer_rules: &Vec<LexerRule>, parser_rules: &Vec<ParserRule>) -> Rule {
    let parser_rule = parser_rules.iter().find(|r| {
        return match r {
            ParserRule::Recursive{name, ..} => name == &token,
            _ => false
        };
    });
    if let Some(parser_rule) = parser_rule {
        return Rule::Parser(parser_rule.clone());
    }
    let lexer_rule = lexer_rules.iter().find(|r| {
        r.name() == &token
    });
    if let Some(lexer_rule) = lexer_rule {
        return Rule::Lexer(lexer_rule.clone());
    }
    if token == "self" {
        return Rule::Parser(ParserRule::SelfRef);
    }
    panic!("Invalid token during parser load: '{}'", token);
}

fn resolve_lexer_regex(pattern: String, rules: &Vec<LexerRule>) -> Regex {
    let reference = Regex::new(r"(\\?)\$([a-zA-Z0-9_]+)").unwrap();
    let output = reference.replace_all(&pattern, |caps: &regex::Captures| {
        if (&caps[1]).len() == 1 {
            return caps[0][1..caps[0].len()].to_string();
        }
        let name = caps.get(2).unwrap().as_str();
        let rule = rules.iter().find(|r| {
            return match r {
                LexerRule::RegexMatch{name: re_name, ..} => re_name == name,
                LexerRule::Capture{name: re_name, ..} => re_name == name,
                LexerRule::Ignore{name: re_name, ..} => re_name == name,
                _ => false
            };
        }).unwrap();
        let re = rule.regex_opt();
        match re {
            Some(re) => {
                let str = re.as_str();
                "(?:".to_owned() + &str[1..str.len()] + ")"
            },
            None => panic!("Invalid reference: {} in lexer pattern", name)
        }
    });
    Regex::new(&*("^".to_owned() + &output.to_string())).unwrap()
}

fn json_to_data(path: &str) -> Data {
    println!("Loading grammar from {}", path);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&format!("Failed to read {}", path));
    serde_json::from_str(&contents).expect(&format!("Failed to parse {}", path))
}

fn data_to_grammar(mut data: Data) -> Grammar {
    let mut grammar = Grammar::new();
    let lexer_rules = data.compute_lexer_rules();
    let parser_rules = data.compute_parser_rules(lexer_rules.clone());
    grammar.set_lexer_rules(lexer_rules);
    grammar.set_parser_rules(parser_rules);
    grammar
}

pub fn read_grammar(path: &str) -> Grammar {
    let data = json_to_data(path);
    data_to_grammar(data)
}

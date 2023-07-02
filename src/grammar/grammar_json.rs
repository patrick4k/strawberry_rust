use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::{Regex};
use crate::grammar::grammar::{Grammar, LexerRule};
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    lexer_rules: Vec<LexerData>
}

#[derive(Clone, Deserialize)]
struct LexerData {
    name: String,
    method: String,
    pattern: String
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
        }).unwrap_or_else(|| panic!("Invalid reference: {} in lexer pattern: {}", name, pattern));
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

fn json_to_data(path: Box<Path>) -> Data {
    println!("Loading grammar from {:?}", path);
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&format!("Failed to read {:?}", path));
    let lexer_rules = serde_json::from_str(&contents).expect(&format!("Failed to parse {:?}", path));
    Data{ lexer_rules }
}

fn data_to_grammar(data: Data) -> Grammar {
    let mut grammar = Grammar::new();
    let lexer_rules = data.compute_lexer_rules();
    grammar.set_lexer_rules(lexer_rules);
    grammar
}

pub fn read_grammar(path: Box<Path>) -> Grammar {
    let data = json_to_data(path);
    data_to_grammar(data)
}

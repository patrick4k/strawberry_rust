use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::Read;
use std::mem::take;
use std::rc::Rc;
use fxhash::FxHashMap;
use regex::{Captures, Regex};
use crate::grammar::grammar::{Grammar, LexerRule, Rule};
use serde::Deserialize;

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
    method: String,
    pattern: String
}

impl Data {
    pub fn to_rules(&self) -> Vec<LexerRule> {
        let mut lexer_rules = vec![];
        for data in &self.lexer_rules {
            let mut cap_group: usize = 0;
            let name = data.name.clone();
            let pattern = data.pattern.clone();
            let method = data.method.clone();
            let rule = match &*method {
                "Match" => LexerRule::Match(name, pattern),
                "RegexMatch" => LexerRule::RegexMatch(name, resolve_lexer_regex(pattern, &lexer_rules)),
                "Ignore" => LexerRule::Ignore(name, resolve_lexer_regex(pattern, &lexer_rules)),
                _ => {
                    let captures = Regex::new(r"Capture\((\d+)\)").unwrap().captures(&data.method);
                    match captures {
                        Some(captures) => {
                            cap_group = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let re = resolve_lexer_regex(data.pattern.clone(), &lexer_rules);
                            LexerRule::Capture(data.name.clone(), re, cap_group)
                        }
                        None => {
                            panic!("Invalid method: {}", method);
                        }
                    }
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
                LexerRule::RegexMatch(re_name, re) => re_name == name,
                _ => false
            };
        }).unwrap();
        let re = rule.regex().as_str();
        "(?:".to_owned() + &re[1..re.len()] + ")"
    });
    println!("{} -> {}", pattern, output);
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
    let rules = data.to_rules();
    grammar.set_lexer_rules(rules);
    grammar
}


pub fn read_grammar(path: &str) -> Grammar {
    let data = json_to_data(path);
    data_to_grammar(data)
}

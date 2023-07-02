use std::path::Path;
use regex::Regex;
use crate::grammar::grammar_json::read_grammar;
use crate::util::util::OneOrMore;

pub struct Grammar {
    lexer_rules: Vec<LexerRule>,
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            lexer_rules: vec![],
        }
    }

    pub fn new_from_json(path: Box<Path>) -> Grammar {
        read_grammar(path)
    }

    pub fn add_rule(&mut self, rule: LexerRule) {
        self.lexer_rules.insert(0, rule)
    }

    pub fn lexer_rules(&self) -> &Vec<LexerRule> {
        &self.lexer_rules
    }
    pub fn set_lexer_rules(&mut self, lexer_rules: Vec<LexerRule>) {
        self.lexer_rules = lexer_rules;
    }
}

#[derive(Clone)]
pub enum LexerRule {
    Match { name: String, pattern: String },
    RegexMatch { name: String, pattern: Regex },
    Ignore { name: String, pattern: Regex },
    Capture { name: String, pattern: Regex, capture: usize },
}

impl LexerRule {
    pub fn name(&self) -> &str {
        match self {
            LexerRule::Match { name, .. } => name,
            LexerRule::RegexMatch { name, .. } => name,
            LexerRule::Ignore { name, .. } => name,
            LexerRule::Capture { name, ..} => name
        }
    }

    pub fn regex(&self) -> &Regex {
        match self.regex_opt() {
            Some(re) => re,
            None => panic!("LexerRule::regex() called on non-regex rule")
        }
    }

    pub fn regex_opt(&self) -> Option<&Regex> {
        match self {
            LexerRule::RegexMatch { pattern, .. } => Some(pattern),
            LexerRule::Ignore { pattern, .. } => Some(pattern),
            LexerRule::Capture { pattern, .. } => Some(pattern),
            _ => None
        }
    }
}

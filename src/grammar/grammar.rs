use regex::Regex;
use crate::grammar::grammar_json::read_grammar;
use crate::parser::parser::RuleCtx;
use crate::util::util::OneOrMore;

pub struct Grammar {
    lexer_rules: Vec<LexerRule>,
    parser_rules: Vec<ParserRule>
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            lexer_rules: vec![],
            parser_rules: vec![]
        }
    }

    pub fn new_from_json(path: &str) -> Grammar {
        read_grammar(path)
    }

    pub fn add_rule(&mut self, rule: Rule) {
        match rule {
            Rule::Lexer(lexer_rule) => self.lexer_rules.insert(0, lexer_rule),
            Rule::Parser(parser_rule) => self.parser_rules.insert(0, parser_rule)
        }
    }

    pub fn lexer_rules(&self) -> &Vec<LexerRule> {
        &self.lexer_rules
    }
    pub fn parser_rules(&self) -> &Vec<ParserRule> {
        &self.parser_rules
    }
    pub fn set_lexer_rules(&mut self, lexer_rules: Vec<LexerRule>) {
        self.lexer_rules = lexer_rules;
    }
    pub fn set_parser_rules(&mut self, parser_rules: Vec<ParserRule>) {
        self.parser_rules = parser_rules;
    }
}

#[derive(Clone)]
pub enum Rule {
    Lexer(LexerRule),
    Parser(ParserRule)
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

#[derive(Clone)]
pub enum ParserRule {
    Pratt { name: String, sequence: Vec<Rule>, left_binding_power: i32 },
    Recursive { name: String, sequence: OneOrMore<Vec<RuleWrapper>> },
    SelfRef
}

#[derive(Clone)]
pub enum RuleWrapper {
    Singleton(Rule),
    Multiton{rule: Rule, min: usize, max: Option<usize>}
}

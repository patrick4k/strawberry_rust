use regex::Regex;
use crate::grammar::grammar_json::read_grammar;
use crate::parser::parser::RuleCtx;

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

pub enum Rule {
    Lexer(LexerRule),
    Parser(ParserRule)
}

pub enum LexerRule {
    Match(String, String),
    RegexMatch(String, Regex),
    Ignore(String, Regex),
    Capture(String, Regex, usize),
}

impl LexerRule {
    pub fn name(&self) -> &str {
        match self {
            LexerRule::Match(name, _) => name,
            LexerRule::RegexMatch(name, _) => name,
            LexerRule::Ignore(name, _) => name,
            LexerRule::Capture(name, _, _) => name
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
            LexerRule::RegexMatch(_, regex) => Some(regex),
            LexerRule::Ignore(_, regex) => Some(regex),
            LexerRule::Capture(_, regex, _) => Some(regex),
            _ => None
        }
    }
}

pub enum ParserRule {
    Match(Vec<Rule>),
    MatchIf(Vec<Rule>, fn(&RuleCtx) -> bool),
}

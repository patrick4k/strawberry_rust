use crate::parser::parser::RuleCtx;
use regex::Regex;

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

    pub fn add_rule(&mut self, rule: Rule) {
        match rule {
            Rule::Lexer(lexer_rule) => self.lexer_rules.push(lexer_rule),
            Rule::Parser(parser_rule) => self.parser_rules.push(parser_rule)
        }
    }

    pub fn lexer_rules(&self) -> &Vec<LexerRule> {
        &self.lexer_rules
    }

    pub fn parser_rules(&self) -> &Vec<ParserRule> {
        &self.parser_rules
    }
}

impl Clone for Grammar {
    fn clone(&self) -> Self {
        Grammar {
            lexer_rules: self.lexer_rules.clone(),
            parser_rules: self.parser_rules.clone()
        }
    }
}

#[derive(Clone)]
pub enum Rule {
    Lexer(LexerRule),
    Parser(ParserRule)
}

#[derive(Clone)]
pub enum LexerRule {
    Match(String),
    RegexMatch(Regex),
    Ignore(String),
    Capture(Regex, usize),
    Assert(bool, Box<LexerRule>)
}

#[derive(Clone)]
pub enum ParserRule {
    Match(Vec<Rule>)
}

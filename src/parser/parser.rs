use std::fs::File;
use std::rc::Rc;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::Token;
use crate::logger::logger::Logger;



pub enum Rule {
    Visitable{name: String},
    NonVisitable,
}

pub enum MatchResult {
    Matched(Rule),
    NotMatched
}

pub enum ParseResult {
    Success(Vec<RuleCtx>),
    Failure(String)
}

pub struct RuleCtx {
    rule: Rule,
    text: String,
    children: Vec<RuleCtx>,

}

impl RuleCtx {
    pub fn rule(&self) -> &Rule {
        &self.rule
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn children(&self) -> &Vec<RuleCtx> {
        &self.children
    }
}

pub struct Parser {
    grammar: Rc<Grammar>,
}

impl Parser {

    pub fn new(grammar: Rc<Grammar>) -> Parser {
        Parser {
            grammar
        }
    }

    pub fn parse(&self, token_stream: Vec<Token>) -> ParseResult {
        if token_stream.len() == 0 {
            return ParseResult::Failure(String::from("Empty stream"));
        }
        ParseResult::Failure(String::from("Parser not implemented"))
    }
}

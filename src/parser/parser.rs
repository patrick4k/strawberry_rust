use std::fs::File;
use std::rc::Rc;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::Token;
use crate::logger::logger::Logger;

pub enum Rule {
    Visitable(String),
    NonVisitable,
    Ignore
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
    pub(crate) rule: Rule,
    text: String,
    pub(crate) children: Vec<RuleCtx>,
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

    pub fn parse(&self, stream: Vec<Token>) -> ParseResult {
        ParseResult::Failure(String::from("Parser not implemented"))
    }
}

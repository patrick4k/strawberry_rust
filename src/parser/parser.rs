use std::fs::File;
use std::rc::Rc;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::Token;
use crate::logger::logger::Logger;

pub struct Parser {
    grammar: Rc<Grammar>,
    logger: Logger
}

// TODO: Implement Parser
impl Parser {

    pub fn new_log_to_file(grammar: Rc<Grammar>, filename: &str) -> Parser {
        Parser {
            grammar,
            logger: Logger::new(File::create(filename).unwrap())
        }
    }

    pub fn parse<T>(&mut self, stream: Vec<Token>) -> ParseResult<T> {
        ParseResult::Failure(String::from("Parser not implemented"))
    }

    pub fn visit<T>(&self, rule: Box<dyn RuleCtx<T>>) -> T {
        rule.visit()
    }
}

pub enum ParseResult<T> {
    Success(Vec<Box<dyn RuleCtx<T>>>),
    Failure(String)
}

pub trait RuleCtx<T> {
    fn visit(&self) -> T;
}

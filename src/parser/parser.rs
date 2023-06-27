use std::fs::File;
use std::mem::take;
use std::rc::Rc;
use crate::grammar::grammar::{Grammar, Rule, LexerRule, ParserRule};
use crate::lexer::lexer::Token;
use crate::logger::logger::Logger;
use crate::parser::parser::PeekResult::NotMatched;

pub enum ParseResult {
    Success(Vec<RuleCtx>),
    Failure(String)
}

pub struct RuleCtx {
    rule: String,
    text: String,
    children: Vec<RuleCtx>,
}

impl RuleCtx {
    pub fn rule(&self) -> &String {
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

    pub fn parse(&self, mut token_stream: Vec<Token>) -> ParseResult {
        let mut logger = Logger::new_file("logs\\parser.log");
        logger.logln("Beginning parsing token stream");

        let mut peeker = ParsePeeker {
            token_stream: &token_stream,
            rules: &self.grammar.parser_rules(),
            index: 0,
        };
        peeker.parse()
    }
}

enum PeekResult {
    Matched(usize),
    NotMatched,
}

struct ParsePeeker<'a, 'b> {
    token_stream: &'a Vec<Token>,
    rules: &'b Vec<ParserRule>,
    index: usize,
}

impl ParsePeeker<'_, '_> {
    fn parse(&mut self) -> ParseResult {

        for rule in self.rules {
            match self.peek(rule) {
                PeekResult::Matched(i) => {
                    return ParseResult::Success(self.get_ctx_stream());
                },
                NotMatched => {}
            }
        }

        ParseResult::Failure("Parser not implemented".to_string())
    }

    fn get_ctx_stream(&self) -> Vec<RuleCtx> {
        todo!()
    }

    fn peek(&mut self, rule: &ParserRule) -> PeekResult {
        todo!()
    }
}

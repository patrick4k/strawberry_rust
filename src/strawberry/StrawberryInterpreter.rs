use std::rc::Rc;
use fxhash::FxHashMap;
use regex::Regex;
use crate::grammar::grammar::{Grammar, LexerRule, Rule};
use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::{Lexer, Token, LexerResult};
use crate::parser::parser::{Parser, ParseResult, RuleCtx};

type ReturnType = f32;

pub struct StrawberryInterpreter {
    lexer: Lexer,
    parser: Parser,
    visitor: FxHashMap<String, fn(RuleCtx) -> ReturnType>
}

impl StrawberryInterpreter {

    pub fn new() -> StrawberryInterpreter {
        let grammar = Rc::new(strawberry_grammar());
        StrawberryInterpreter {
            lexer: Lexer::new(Rc::clone(&grammar)),
            parser: Parser::new(Rc::clone(&grammar)),
            visitor: Default::default(),
        }
    }
}

impl Interpreter<ReturnType> for StrawberryInterpreter {
    fn get_lexer(&self) -> &Lexer {
        &self.lexer
    }

    fn get_parser(&self) -> &Parser {
        &self.parser
    }

    fn get_visit_map(&self) -> FxHashMap<String, fn(&RuleCtx) -> ReturnType> {
        todo!()
    }
}

fn strawberry_grammar() -> Grammar {
    Grammar::new_from_json(r"C:\Users\Patrick\Documents\Code\Strawberry\strawberry_rust\src\strawberry\grammar.json")
}

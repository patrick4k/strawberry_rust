use crate::gen::tokens::Token;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::{Lexer, LexerResult};
use crate::parser::parser::{Parser, ParseResult};

pub enum InterpreterResult {
    Success,
    Failure(String)
}

pub trait Interpreter {

    fn get_grammar(&self) -> Grammar;
    fn process_args(&mut self, args: Vec<String>) {    }
    fn parse(&mut self, tokens: Vec<Token>) -> ParseResult {
        ParseResult::Failure("Parser not implemented".to_string())
    }
    fn interpret(&mut self) -> InterpreterResult {
        InterpreterResult::Failure("Interpreter not implemented".to_string())
    }

    fn lex(&self, stream: &str) -> LexerResult {
        let lexer = Lexer{grammar: &self.get_grammar() };
        lexer.tokenize(stream)
    }

    fn execute_from_file(&mut self, path: &str) {
        let stream = std::fs::read_to_string(path).unwrap();
        self.execute(&stream);
    }

    fn execute(&mut self, stream: &str) {
        let lexer_result = self.lex(stream);
        match lexer_result {
            LexerResult::Success(rules) => {

                let parser_result = self.parse(rules);
                match parser_result {
                    ParseResult::Success => {
                        println!("SUCCESS: Parser successfully parsed stream");
                    }
                    ParseResult::Failure(msg) => {
                        println!("ERROR: Parser failed to parse stream: {}", msg);
                    }
                }
            }
            LexerResult::Failure(msg) => {
                println!("ERROR: Lexer failed to tokenize stream: {}", msg);
            }
        }
    }
}

use crate::gen::tokens::Token;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::{Lexer, LexerResult};

pub enum InterpreterResult {
    Success,
    Failure(String)
}

pub trait Interpreter {

    fn get_grammar(&self) -> Grammar;
    fn process_args(&mut self, args: Vec<String>) {    }
    fn parse(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        Err("Parser not implemented".to_string())
    }
    fn interpret(&mut self) -> InterpreterResult {
        InterpreterResult::Failure("Interpreter not implemented".to_string())
    }

    fn lex(&self, stream: &str) -> LexerResult {
        let lexer = Lexer{grammar: &self.get_grammar() };
        lexer.tokenize(stream)
    }

    fn execute_from_file(&mut self, path: &str) -> InterpreterResult {
        let stream = std::fs::read_to_string(path).unwrap();
        self.execute(&stream)
    }

    fn execute(&mut self, stream: &str) -> InterpreterResult {
        let lexer_result = self.lex(stream);
        match lexer_result {
            LexerResult::Success(rules) => {

                let parser_result = self.parse(rules);
                match parser_result {
                    Ok(()) => {
                        println!("SUCCESS: Parser successfully parsed stream");
                        return self.interpret();
                    }
                    Err(msg) => {
                        println!("ERROR: Parser failed to parse stream: {}", msg);
                    }
                }
            }
            LexerResult::Failure(msg) => {
                println!("ERROR: Lexer failed to tokenize stream: {}", msg);
            }
        }
        InterpreterResult::Failure("Interpreter failed to execute".to_string())
    }
}

use crate::lexer::grammar::Grammar;
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser::Parser;

pub struct Interpreter {
    lexer: Tokenizer,
    parser: Parser
}

impl Interpreter {

    pub fn new() -> Interpreter {
        let grammar = Grammar::new();
        Interpreter {
            lexer: Tokenizer::new_log_to_file(grammar, "logs\\lexer.log"),
            parser: Parser::new()
        }
    }

    pub fn new_thread(&mut self, stream: &str) {
        self.lexer.tokenize(stream);
    }
}

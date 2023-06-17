use std::env;
use strawberry::interpreter::Interpreter;
use crate::lexer::grammar::Grammar;
use crate::lexer::tokenizer::Tokenizer;

mod lexer;
mod logger;
mod parser;
mod strawberry;

fn main() {

    // Init Interpreter
    let mut interpreter = Interpreter::new();

    // Process args
    let args: Vec<_> = env::args().collect();
    for arg in &args {
        // TODO: interpreter.process_arg(arg);
    }

    interpreter.new_thread("lexer log");
}

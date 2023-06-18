use std::env;
use regex::Regex;
use strawberry::interpreter::Interpreter;
use crate::grammar::grammar::Grammar;
use crate::lexer::lexer::Lexer;

mod lexer;
mod logger;
mod parser;
mod grammar;
mod strawberry;

fn main() {

    // Init Interpreter
    let mut interpreter = Interpreter::new();

    // Process args
    let args: Vec<_> = env::args().collect();
    for arg in &args {
        // TODO: interpreter.process_arg(arg);
    }

    interpreter.execute("letlet;let");
}

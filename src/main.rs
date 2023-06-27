extern crate core;

use std::env;
use regex::Regex;
use strawberry::StrawberryInterpreter::StrawberryInterpreter;
use crate::grammar::grammar::Grammar;
use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;

mod lexer;
mod logger;
mod parser;
mod grammar;
mod interpreter;
mod strawberry;
mod util;

fn main() {

    // Init Interpreter
    let mut interpreter = StrawberryInterpreter::new();

    // Process args
    let args: Vec<_> = env::args().collect();
    interpreter.process_args(args);

    // Execute from file
    interpreter.execute_from_file(r"C:\Users\Patrick\Documents\Code\Strawberry\strawberry_rust\test\test_stream.sb");
}

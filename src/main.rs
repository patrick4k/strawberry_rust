extern crate core;

use std::env;
use strawberry::strawberry_interpreter::StrawberryInterpreter;
use crate::interpreter::interpreter::Interpreter;

mod lexer;
mod logger;
mod parser;
mod grammar;
mod interpreter;
mod strawberry;
mod util;
mod gen;

fn main() {
    let mut interpreter = StrawberryInterpreter::new();

    let args: Vec<_> = env::args().collect();
    interpreter.process_args(args);

    interpreter.exec();
}

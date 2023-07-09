extern crate core;

use std::env;
use strawberry::strawberry_interpreter::StrawberryInterpreter;
use crate::interpreter::interpreter::{Interpreter, InterpreterResult};

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

    let args = env::args().collect();
    interpreter.process_args(args);

    match interpreter.exec() {
        InterpreterResult::Success => {
            println!("SUCCESS: Interpreter successfully executed");
        }
        InterpreterResult::Failure(msg) => {
            println!("ERROR: Interpreter failed to execute: {}", msg);
        }
    }
}

use std::path::Path;
use crate::gen::tokens::Token;
use crate::grammar::grammar::Grammar;
use crate::interpreter::interpreter::{Interpreter, InterpreterResult};
use crate::strawberry::parser::ast::Script;
use crate::strawberry::parser::script;

pub struct StrawberryInterpreter {
    path: Option<String>,
    grammar_path: Option<String>,
    ast: Option<Script>
}

impl StrawberryInterpreter {
    pub fn new() -> StrawberryInterpreter {
        StrawberryInterpreter {
            path: None,
            grammar_path: None,
            ast: None
        }
    }

    pub fn exec(&mut self) -> InterpreterResult {
        if let Some(path) = &self.path.clone() {
            return self.execute_from_file(path.as_str());
        }
        panic!("No path specified");
    }
}

impl Interpreter for StrawberryInterpreter {
    fn get_grammar(&self) -> Grammar {
        if let Some(grammar_path) = &self.grammar_path {
            return Grammar::new_from_json(Box::from(Path::new(grammar_path.as_str())));
        }
        panic!("No grammar path specified");
    }

    fn process_args(&mut self, args: Vec<String>) {
        if let Some(arg) = args.get(1) {
            self.path = Some(arg.to_string());
        }
        if let Some(arg) = args.get(2) {
            self.grammar_path = Some(arg.to_string());
        }
    }

    fn parse(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        let script_node= script::parse(tokens);
        match script_node {
            Ok(script) => {
                self.ast = Some(script);
                return Ok(());
            }
            Err(msg) => Err(msg)
        }
    }

    fn interpret(&mut self) -> InterpreterResult {
        InterpreterResult::Failure("Interpreter not implemented".to_string())
    }
}

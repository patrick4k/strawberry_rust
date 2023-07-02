use std::cell::{Ref, RefCell};
use std::env::remove_var;
use std::mem::take;
use std::path::Path;
use std::ptr::replace;
use crate::gen::tokens::Token;
use crate::grammar::grammar::Grammar;
use crate::interpreter::interpreter::{Interpreter, InterpreterResult};
use crate::parser::ast::AST;
use crate::parser::parser::{Parser, ParseResult, ParserVisitor, SubParseResult};
use crate::parser::parser::ParseResult::Failure;
use crate::strawberry::parser::rule::Script;
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

    pub fn exec(&mut self) {
        if let Some(path) = &self.path.clone() {
            self.execute_from_file(path.as_str());
        }
        else {
            panic!("No path specified");
        }
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

    fn parse(&mut self, tokens: Vec<Token>) -> ParseResult {
        let tokens = std::rc::Rc::new(RefCell::new(tokens));
        let script_node= script::parse(tokens);
        match script_node {
            SubParseResult::Success(node) => {
                println!("SUCCESS: Parser successfully parsed stream");
                self.ast = Some(node.rule);
                return ParseResult::Success;
            }
            SubParseResult::Failure(msg) => {
                Failure(msg)
            }
        }
    }

    fn interpret(&mut self) -> InterpreterResult {
        if let Some(script) = self.ast.clone() {
            self.visit(script);
        }
        todo!()
    }
}

type RetVal = i32;
impl ParserVisitor<Script, RetVal> for StrawberryInterpreter {
    fn visit(&mut self, rule: Script) -> RetVal {

        todo!()
    }
}

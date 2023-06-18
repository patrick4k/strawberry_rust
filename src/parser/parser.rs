use std::fs::File;
use crate::lexer::grammar::{Grammar, Rule};
use crate::logger::logger::Logger;

pub struct Parser {
    grammar: Grammar,
    logger: Logger
}

// TODO: Implement Parser
impl Parser {
    pub fn new() -> Parser {
        Parser {
            grammar: Grammar::new(),
            logger: Logger::new_console()
        }
    }

    pub fn new_log_to_file(grammar: Grammar, filename: &str) -> Parser {
        Parser {
            grammar,
            logger: Logger::new(File::create(filename).unwrap())
        }
    }
}

pub trait RuleCtx {
    fn get_children(&self) -> Vec<Box<dyn RuleCtx>>;
    fn get_text(&self) -> &str;
    fn get_rule(&self) -> Rule;
}

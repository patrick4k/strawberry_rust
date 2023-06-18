use std::rc::Rc;
use regex::Regex;
use crate::grammar::grammar::{Grammar, LexerRule, Rule};
use crate::lexer::lexer::{Lexer, Token, TokenizeResult};
use crate::parser::parser::{Parser, ParseResult};

pub struct Interpreter {
    lexer: Lexer,
    parser: Parser
}

impl Interpreter {

    pub fn new() -> Interpreter {
        let logs = ("logs\\lexer.log", "logs\\parser.log");
        let grammar = Rc::new(strawberry_grammar());
        Interpreter {
            lexer: Lexer::new_log_to_file(Rc::clone(&grammar), logs.0),
            parser: Parser::new_log_to_file(Rc::clone(&grammar), logs.1)
        }
    }

    pub fn execute(&mut self, stream: &str) {
        self.lex(stream);
    }

    fn lex(&mut self, stream: &str) {
        let tokenized_result = self.lexer.tokenize(stream);
        match tokenized_result {
            TokenizeResult::Success(tokens) => {
                self.parse(tokens);
            }
            TokenizeResult::Failure(err) => {
                println!("{}", err);
            }
        }
    }

    fn parse(&mut self, tokens: Vec<Token>) {
        let parsed_result: ParseResult<f32> = self.parser.parse(tokens);
        match parsed_result {
            ParseResult::Success(rules) => {
                for rule in rules {
                    // TODO: Implement async on visit
                    self.parser.visit(rule);
                }
            }
            ParseResult::Failure(err) => {
                println!("ERROR: {}", err);
            }
        }
    }
}

fn strawberry_grammar() -> Grammar {
    let mut grammar = Grammar::new();
    let lexer_rules: Vec<&str> = vec![
        "(?:let)+", // Lets
        r"[a-zA-Z](\w|_)*", // Identifier
        "=", // Equals
        ";", // Semicolon
    ];

    for rule in lexer_rules {
        let re = Regex::new(&*("^".to_owned() + rule)).unwrap();
        let lr = LexerRule::RegexMatch(re);
        grammar.add_rule(Rule::Lexer(lr));
    }
    grammar
}

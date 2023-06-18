use regex::Regex;
use crate::lexer::grammar::{Grammar, LexerRule, Rule};
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

pub struct Interpreter {
    lexer: Lexer,
    parser: Parser
}

impl Interpreter {

    pub fn new() -> Interpreter {
        let logs = ("logs\\lexer.log", "logs\\parser.log");
        let grammar = strawberry_grammar();
        Interpreter {
            lexer: Lexer::new_log_to_file(grammar.clone(), logs.0),
            parser: Parser::new_log_to_file(grammar, logs.1)
        }
    }

    pub fn new_thread(&mut self, stream: &str) {
        self.lexer.tokenize(stream);
    }
}

struct RetVal {
    value: String
}

fn strawberry_grammar() -> Grammar {
    let mut grammar = Grammar::new();
    let lexer_rules: Vec<&str> = vec![
        "let", // Let
        r"[a-zA-Z](?:\w|_)*", // Identifier
        "=", // Equals
        ";", // Semicolon
    ];

    for rule in lexer_rules {
        let re = Regex::new(rule).unwrap();
        let lr = LexerRule::RegexMatch(re);
        grammar.add_rule(Rule::Lexer(lr));
    }

    grammar
}

fn strawberry_parser() -> Parser {
    // TODO: Build parser
    Parser::new()
}

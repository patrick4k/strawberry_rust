use std::borrow::BorrowMut;
use fxhash::FxHashMap;
use crate::lexer::lexer::{Lexer, Token, LexerResult};
use crate::parser::parser::{Parser, ParseResult, Rule, RuleCtx};

pub trait Interpreter<T> {

    fn get_lexer(&self) -> &Lexer;
    fn get_parser(&self) -> &Parser;
    fn get_visit_map(&self) -> FxHashMap<String, fn(&RuleCtx)->T>;

    fn process_args(&self, args: Vec<String>) {    }

    fn aggregate(&self, previous: Option<T>, next: Option<T>) -> Option<T> {
        next
    }

    fn lex(&self, stream: &str) -> LexerResult {
        let lexer = self.get_lexer();
        lexer.tokenize(stream)
    }

    fn parse(&self, tokens: Vec<Token>) -> ParseResult {
        let parser = self.get_parser();
        parser.parse(tokens)
    }

    fn execute_from_file(&mut self, path: &str) {
        let stream = std::fs::read_to_string(path).unwrap();
        self.execute(&stream);
    }

    fn execute(&mut self, stream: &str) {
        let lexer_result = self.lex(stream);
        match lexer_result {
            LexerResult::Success(rules) => {
                let parser_result = self.parse(rules);
                match parser_result {
                    ParseResult::Success(ctx_list) => {
                        self.visit_ctx_list(&ctx_list);
                    }
                    ParseResult::Failure(msg) => {
                        println!("ERROR: Parser failed to parse stream: {}", msg);
                    }
                }
            }
            LexerResult::Failure(msg) => {
                println!("ERROR: Lexer failed to tokenize stream: {}", msg);
            }
        }
    }

    fn visit_ctx_list(&self, ctx_list: &Vec<RuleCtx>) {
        for ctx in ctx_list {
            self.visit_ctx(ctx);
        }
    }

    fn visit_ctx(&self, ctx: &RuleCtx) -> Option<T> {
        match &ctx.rule() {
            Rule::Visitable{name} => {
                let visit_map = self.get_visit_map();
                return match visit_map.get(&*name) {
                    Some(visit_fn) => {
                        Some(visit_fn(ctx))
                    }
                    None => {
                        println!("ERROR: No visit function found for rule: {}", name);
                        None
                    }
                }
            }
            Rule::NonVisitable => {
                return self.visit_children(ctx);
            }
        }
        None
    }

    fn visit_children(&self, ctx: &RuleCtx) -> Option<T> {
        let mut ret_val: Option<T> = None;
        for child in ctx.children() {
            let tmp_val = self.visit_ctx(child);
            ret_val = self.aggregate(ret_val, tmp_val);
        }
        ret_val
    }
}

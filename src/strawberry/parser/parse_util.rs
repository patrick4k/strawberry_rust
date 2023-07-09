use std::mem::replace;
use crate::gen::tokens::Token;
use crate::strawberry::parser::ast_visits::Rule;

pub struct SubParse<T: Rule> {
    pub(crate) rule: T,
    tokens: Vec<Token>,
    state: SubParseState
}

enum SubParseState {
    Init,
    HasTokens,
    Empty,
    Failure
}

impl<T: Rule> SubParse<T> {
    pub fn new(rule: T) -> SubParse<T> {
        SubParse {
            rule,
            tokens: vec![],
            state: SubParseState::Init
        }
    }

    pub fn from(rule: T, tokens: Vec<Token>) -> SubParse<T> {
        SubParse {
            rule,
            tokens,
            state: SubParseState::HasTokens
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        replace(&mut self.tokens, vec![])
    }
    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }
}

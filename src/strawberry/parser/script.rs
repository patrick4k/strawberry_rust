use std::cell::RefCell;
use std::rc::Rc;
use crate::gen::tokens::Token;
use crate::parser::parser::{SubParseResult};
use crate::strawberry::parser::rule::Script;

type TokenList = Rc<RefCell<Vec<Token>>>;

pub fn parse(tokens: TokenList) -> SubParseResult<Script> {
    SubParseResult::Failure("Script parser not implemented".to_string())
}

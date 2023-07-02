use crate::gen::tokens::Token;
use crate::parser::ast::ASTNode;

pub trait Parser {
    fn parse(&self, token_stream: Vec<Token>) -> ParseResult;
}

pub enum ParseResult {
    Success,
    Failure(String)
}

impl ParseResult {
    pub fn is_success(&self) -> bool {
        match self {
            ParseResult::Success => true,
            ParseResult::Failure(_) => false,
        }
    }
}

pub enum SubParseResult<T> {
    Success(ASTNode<T>),
    Failure(String)
}

pub trait ParserVisitor<RuleType, RetVal> {
    fn visit(&mut self, rule: RuleType) -> RetVal;
}

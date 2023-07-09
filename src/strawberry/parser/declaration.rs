use crate::gen::tokens::Token;
use crate::strawberry::parser::ast::Declaration;
use crate::strawberry::parser::parse_util::SubParse;

enum DecType {
    FnDeclaration,
    StateDeclaration
}

struct DeclarationBuilder {
    dec_type: DecType,
}

pub fn parse_declaration(mut tokens: Vec<Token>) -> Result<SubParse<Declaration>, String> {
    let mut i: usize = 0;
    if let Some(token) = tokens.get(i) {
        match token {

            _ => {}
        }
    }

    Err("Not implemented".into())
}

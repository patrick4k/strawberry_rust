use crate::gen::tokens::Token;
use crate::strawberry::parser::ast::{Script};
use crate::strawberry::parser::declaration::parse_declaration;
use crate::util::util::ZeroOrMore;

impl Script {
    pub fn new() -> Script {
        Script {
            declarations: ZeroOrMore::Zero,
            body: ZeroOrMore::Zero
        }
    }
}

pub fn parse(mut tokens: Vec<Token>) -> Result<Script, String> {
    let mut script = Script::new();
    while let Ok(mut sub_parse) = parse_declaration(tokens) {
        script.declarations.push(sub_parse.rule.clone());
        tokens = sub_parse.tokens();
    }
    Ok(script)
}

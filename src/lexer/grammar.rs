
pub struct Grammar {
    tokens: Vec<Token>
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            tokens: vec![]
        }
    }
}

pub trait LexerRule {
    fn get_regex(&self) -> &str;
}

enum Token {

}

impl LexerRule for Token {
    fn get_regex(&self) -> &str {
        todo!()
    }
}

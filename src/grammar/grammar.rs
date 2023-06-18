use regex::Regex;

pub struct Grammar {
    lexer_rules: Vec<LexerRule>,
    parser_rules: Vec<ParserRule>
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            lexer_rules: vec![],
            parser_rules: vec![]
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        match rule {
            Rule::Lexer(lexer_rule) => self.lexer_rules.push(lexer_rule),
            Rule::Parser(parser_rule) => self.parser_rules.push(parser_rule)
        }
    }

    pub fn lexer_rules(&self) -> &Vec<LexerRule> {
        &self.lexer_rules
    }

    pub fn parser_rules(&self) -> &Vec<ParserRule> {
        &self.parser_rules
    }
}

pub enum Rule {
    Lexer(LexerRule),
    Parser(ParserRule)
}

pub enum LexerRule {
    Match(String),
    RegexMatch(Regex),
    Ignore(Regex),
    Capture(Regex, usize),
}

pub enum ParserRule {
    Match(Vec<Rule>)
}

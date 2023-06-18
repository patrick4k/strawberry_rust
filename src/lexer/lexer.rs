use std::{io::stdout, fs::File};
use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};
use regex::Regex;
use crate::lexer::grammar::{Grammar, LexerRule};
use crate::logger::logger::Logger;

pub struct Lexer {
    grammar: Grammar,
    logger_path: String
}

impl Lexer {
    pub fn new(grammar: Grammar) -> Lexer {
        Lexer {
            grammar,
            logger_path: String::from("lexer.log")
        }
    }

    pub fn new_log_to_file(grammar: Grammar, filename: &str) -> Lexer {
        Lexer {
            grammar,
            logger_path: String::from(filename)
        }
    }

    fn log(&self, text: &str) {
        let mut logger = Logger::new(File::create(&self.logger_path).unwrap());
        logger.log(text);
    }

    fn logln(&self, text: &str) {
        // let mut logger = Logger::new(File::create(&self.logger_path).unwrap());
        let mut logger = Logger::new_console();
        logger.logln(text);
    }

    pub fn tokenize(&mut self, stream: &str) -> TokenizeResult {

        // TODO: Proof of concept, need cleaning up

        self.logln("Beginning tokenizing stream: ");
        self.logln(stream);

        let mut substream = stream[0..stream.len()].to_string();

        let mut tokens: Vec<Token> = vec![];
        let mut char_count: usize = 0;
        'stream_iter:
        while !substream.is_empty() {
            'rule_iter:
            for rule in self.grammar.lexer_rules() {
                match rule {
                    LexerRule::Match(text) => {
                        self.logln(&format!("Attempting to match: {}", text));
                        if stream.starts_with(&*text) {
                            self.logln("Matched!");
                            let chars = text.len();
                            char_count += chars;
                            tokens.push(Token {
                                rule: rule.clone(),
                                text: text.to_string()
                            });
                            break 'rule_iter;
                        }
                    },
                    LexerRule::RegexMatch(regex) => {
                        self.logln(&format!("Attempting to regex match: {}", regex));
                        if let Some(caps) = regex_match(regex, substream, 0) {
                            self.logln(&format!("Matched! Captured: {}", caps));
                            let text = caps;
                            let chars = text.len();
                            char_count += chars;
                            tokens.push(Token {
                                rule: rule.clone(),
                                text
                            });
                            self.logln(&format!("Returning success with {} chars", chars));
                            break 'rule_iter;
                        }
                        self.logln(&format!("Failed to match: {}", regex));

                    }
                    LexerRule::Ignore(_) => {}
                    LexerRule::Capture(regex, capture) => {
                        self.logln(&format!("Attempting to regex match: {}", regex));
                        if let Some(caps) = regex_match(regex, substream, *capture) {
                            self.logln(&format!("Matched! Captured: {}", caps));
                            let text = caps;
                            let chars = text.len();
                            char_count += chars;
                            tokens.push(Token {
                                rule: rule.clone(),
                                text
                            });
                            break 'rule_iter;
                        }
                    }
                    LexerRule::Assert(assert, rule) => {}
                }
                self.logln("No match found, returning failure");
                return TokenizeResult::Failure(String::from("No matching rule found"));
            }
            if char_count == stream.len() {
                self.logln("Reached end of stream");
                break 'stream_iter;
            }
            if char_count > stream.len() {
                self.logln("Error: char_count > stream.len()");
                return TokenizeResult::Failure(String::from("char_count > stream.len()"));
            }
            substream = stream[char_count..stream.len()].to_string();
            self.logln(&format!("Substream: {}\n", substream));
        }
        TokenizeResult::Failure(String::from("Not implemented"))
    }
}

fn regex_match(regex: &Regex, stream: String, capture: usize) -> Option<String> {
    if let Some(caps) = regex.captures(&*stream) {
        return Some(caps.get(capture).unwrap().as_str().to_string());
    }
    None
}

struct Token {
    rule: LexerRule,
    text: String
}

// TODO
pub enum TokenizeResult {
    Success(Vec<Token>),
    Failure(String)
}

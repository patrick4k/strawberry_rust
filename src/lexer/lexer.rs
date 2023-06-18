use std::{io::stdout, fs::File};
use std::borrow::BorrowMut;
use std::fmt::{Debug, format, Formatter};
use std::rc::Rc;
use regex::Regex;
use crate::grammar::grammar::{Grammar, LexerRule};
use crate::logger::logger::Logger;

pub struct Token {
    rule: LexerRule,
    text: String
}

pub enum MatchResult {
    Matched((Token, usize)),
    Ignored(usize),
    NotMatched
}

pub enum TokenizeResult {
    Success(Vec<Token>),
    Failure(String)
}

pub struct Lexer {
    grammar: Rc<Grammar>,
    logger_path: String
}

impl Lexer {
    pub fn new(grammar: Rc<Grammar>) -> Lexer {
        Lexer {
            grammar,
            logger_path: String::from("lexer.log")
        }
    }

    pub fn new_log_to_file(grammar: Rc<Grammar>, filename: &str) -> Lexer {
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
        let mut logger = Logger::new_console();
        logger.logln(text);
    }

    pub fn tokenize(&mut self, stream: &str) -> TokenizeResult {
        let mut logger = Logger::new_file(&self.logger_path);

        logger.logln(&format!("Beginning tokenizing stream:\n\"{}\"\n", stream));

        let mut substream = stream[0..stream.len()].to_string();

        let mut tokens: Vec<Token> = vec![];
        let mut char_count: usize = 0;

        'stream_iter:
        loop {
            let prev_char_count = char_count;

            'rule_iter:
            for rule in self.grammar.lexer_rules() {

                let result = match rule {
                    LexerRule::Match(text) => get_match(text, &substream, logger.borrow_mut()),
                    LexerRule::RegexMatch(regex) => get_regex_match(regex, &substream, 0, logger.borrow_mut()),
                    LexerRule::Capture(regex, capture) => get_regex_match(regex, &substream, *capture, logger.borrow_mut()),
                    LexerRule::Ignore(regex) => get_regex_ignore(regex, &substream, logger.borrow_mut()),
                };

                match result {
                    MatchResult::Matched((token, chars)) => {
                        tokens.push(token);
                        char_count += chars;
                        break 'rule_iter;
                    }
                    MatchResult::Ignored(chars) => {
                        char_count += chars;
                        break 'rule_iter
                    }
                    MatchResult::NotMatched => { /* Do next rule */ }
                }
            }

            if char_count == stream.len() {
                logger.logln("Reached end of stream");
                break 'stream_iter;
            }

            if prev_char_count == char_count {
                logger.logln("Error: No rules matched");
                return TokenizeResult::Failure(String::from("No rules matched"));
            }

            substream = stream[char_count..stream.len()].to_string();
            logger.logln(&format!("Substream: \"{}\"\n", substream));
        }
        TokenizeResult::Success(tokens)
    }
}

fn get_match(text: &str, stream: &String, logger: &mut Logger) -> MatchResult {
    if stream.starts_with(text) {
        let chars = text.len();
        let token = Token {
            rule: LexerRule::Match(text.to_string()),
            text: text[0..chars].to_string()
        };
        logger.logln(format!("Matched: '{}'", token.text).as_str());
        return MatchResult::Matched((token, chars));
    }
    MatchResult::NotMatched
}

fn get_regex_match(regex: &Regex, stream: &String, capture: usize, logger: &mut Logger) -> MatchResult {
    if let Some(caps) = regex.captures(&*stream) {
        let text = caps.get(capture).unwrap().as_str().to_string();
        let chars = text.len();
        let token = Token {
            rule: LexerRule::RegexMatch(regex.clone()),
            text
        };
        logger.logln(format!("Matched: '{}' to '{}'", token.text, regex.as_str()).as_str());
        return MatchResult::Matched((token, chars));
    }
    MatchResult::NotMatched
}

fn get_regex_ignore(regex: &Regex, stream: &String, logger: &mut Logger) -> MatchResult {
    if let Some(caps) = regex.captures(&*stream) {
        let text = caps.get(0).unwrap().as_str();
        let chars = text.len();
        logger.logln(format!("Ignored: {} chars", chars).as_str());
        return MatchResult::Ignored(chars);
    }
    MatchResult::NotMatched
}

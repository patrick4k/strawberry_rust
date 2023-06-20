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

pub enum LexerResult {
    Success(Vec<Token>),
    Failure(String)
}

pub struct Lexer {
    grammar: Rc<Grammar>,
}

impl Lexer {
    pub fn new(grammar: Rc<Grammar>) -> Lexer {
        Lexer {
            grammar
        }
    }

    fn log(&self, text: &str) {
        let mut logger = Logger::new(File::create("logs\\lexer.log").unwrap());
        logger.log(text);
    }

    fn logln(&self, text: &str) {
        let mut logger = Logger::new_console();
        logger.logln(text);
    }

    pub fn tokenize(&self, stream: &str) -> LexerResult {
        let sep = "----------------------------------------------------------------------------";
        let sep2 = "Substream ..................................................................";
        let mut logger = Logger::new_file("logs\\lexer.log");
        logger.logln(&format!("Beginning tokenizing stream: \n{}\n{}\n{}\n\n", sep, stream, sep));

        let mut substream = stream[0..stream.len()].to_string();
        let mut tokens: Vec<Token> = vec![];
        let mut char_count: usize = 0;

        'stream_iter:
        loop {
            let prev_char_count = char_count;

            'rule_iter:
            for rule in self.grammar.lexer_rules() {

                let result = match rule {
                    LexerRule::Match(name, text) =>
                        get_match(name, text, &substream, logger.borrow_mut()),
                    LexerRule::RegexMatch(name, regex) =>
                        get_regex_match(name, regex, &substream, 0, logger.borrow_mut()),
                    LexerRule::Capture(name, regex, capture) =>
                        get_regex_match(name, regex, &substream, *capture, logger.borrow_mut()),
                    LexerRule::Ignore(name, regex) =>
                        get_regex_ignore(name, regex, &substream, logger.borrow_mut()),
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
                return LexerResult::Failure(String::from("No rules matched"));
            }

            substream = stream[char_count..stream.len()].to_string();
            // logger.logln(&format!("Substream: \n{}\n{}\n{}\n", sep, substream, sep));
            logger.logln(&format!("{}\n{}\n{}\n\n", sep2, substream, sep));
        }
        LexerResult::Success(tokens)
    }
}

fn get_match(name: &String, text: &str, stream: &String, logger: &mut Logger) -> MatchResult {
    if stream.starts_with(text) {
        let chars = text.len();
        let token = Token {
            rule: LexerRule::Match(name.clone(), text.to_string()),
            text: text[0..chars].to_string()
        };
        logger.logln(format!("Matched: '{}'", token.text).as_str());
        return MatchResult::Matched((token, chars));
    }
    MatchResult::NotMatched
}

fn get_regex_match(name: &String, regex: &Regex, stream: &String, capture: usize, logger: &mut Logger) -> MatchResult {
    if let Some(caps) = regex.captures(&*stream) {
        let text = caps.get(capture).unwrap().as_str().to_string();
        let chars = text.len();
        let token = Token {
            rule: LexerRule::RegexMatch(name.clone(), regex.clone()),
            text
        };
        logger.logln(format!("Matched: '{}' to {} = '{}'", token.text, name, regex.as_str()).as_str());
        return MatchResult::Matched((token, chars));
    }
    MatchResult::NotMatched
}

fn get_regex_ignore(name: &String, regex: &Regex, stream: &String, logger: &mut Logger) -> MatchResult {
    if let Some(caps) = regex.captures(&*stream) {
        let text = caps.get(0).unwrap().as_str();
        let chars = text.len();
        logger.logln(format!("Ignored: {} chars to {} = '{}'", chars, name, regex.as_str()).as_str());
        return MatchResult::Ignored(chars);
    }
    MatchResult::NotMatched
}

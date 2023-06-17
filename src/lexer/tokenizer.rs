use crate::lexer::grammar::Grammar;
use crate::logger::logger::Logger;

pub struct Tokenizer {
    grammar: Grammar,
    logger: Logger
}

impl Tokenizer {
    pub fn new(grammar: Grammar) -> Tokenizer {
        Tokenizer {
            grammar,
            logger: Logger::new(std::io::stdout())
        }
    }

    pub fn new_log_to_file(grammar: Grammar, filename: &str) -> Tokenizer {
        Tokenizer {
            grammar,
            logger: Logger::new(std::fs::File::create(filename).unwrap())
        }
    }

    fn log(&mut self, text: &str) {
        self.logger.log(text);
    }

    fn logln(&mut self, text: &str) {
        self.logger.logln(text);
    }

    pub fn tokenize(&mut self, stream: &str) -> Vec<&str> {
        self.logln("Begin tokenizing stream: ");
        self.logln(stream);

        // TODO: Tokenize stream
        return vec![];
    }
}

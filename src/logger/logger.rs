use std::io::{stdout, Write};

pub struct Logger {
    out: Box<dyn Write>
}

impl Logger {

    pub fn new_console() -> Logger {
        Logger {
            out: Box::new(stdout())
        }
    }

    pub fn new<T>(out: T) -> Logger where T: Write + 'static {
        Logger {
            out: Box::new(out)
        }
    }

    pub fn new_file(filename: &str) -> Logger {
        Logger::new(std::fs::File::create(filename).unwrap())
    }

    pub fn log(&mut self, message: &str) {
        self.out.write(message.as_bytes()).unwrap();
    }

    pub fn logln(&mut self, message: &str) {
        self.out.write(message.as_bytes()).unwrap();
        self.out.write("\n".as_bytes()).unwrap();
    }
}

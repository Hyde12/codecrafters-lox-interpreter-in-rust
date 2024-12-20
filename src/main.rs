use std::env;
use std::fs;
use std::io::{self, Write};

struct Scanner {
    source: String,
    tokens: Vec<(String, String, String, i32)>, // token_type, text, literal, line
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner { 
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.current += 1;

            self.scan_token(self.current);
        }

        self.tokens.push((String::from("EOF"), String::from(""), String::from("null"), self.line));
    }

    fn scan_token(&mut self, current: usize) {
        let char = self.source.chars().nth(current - 1).unwrap();
        let null = "null".to_string();
        match char {
            '(' => self.add_token("LEFT_PAREN".to_string(), null),
            ')' => self.add_token("RIGHT_PAREN".to_string(), null),
            '{' => self.add_token("LEFT_BRACE".to_string(), null),
            '}' => self.add_token("RIGHT_BRACE".to_string(), null),
            _ => println!("no!")
        }
    }

    fn add_token(&mut self, token_type: String, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push((token_type, text.to_string(), literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            let mut to_scan = Scanner::new(file_contents);
            to_scan.scan_tokens();

            for token in to_scan.tokens {
                let (token_type, text, literal, _line) = token;
                println!("{token_type} {text} {literal}");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

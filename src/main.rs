use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

struct Scanner {
    source: String,
    tokens: Vec<(String, String, String, i32)>, // token_type, text, literal, line
    start: usize,
    current: usize,
    line: i32,
    errors: usize,
}

impl Scanner { 
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: 0,
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
        match char {
            '(' => self.add_null_token("LEFT_PAREN".to_string()),
            ')' => self.add_null_token("RIGHT_PAREN".to_string()),
            '{' => self.add_null_token("LEFT_BRACE".to_string()),
            '}' => self.add_null_token("RIGHT_BRACE".to_string()),
            ',' => self.add_null_token("COMMA".to_string()),
            '.' => self.add_null_token("DOT".to_string()),
            '-' => self.add_null_token("MINUS".to_string()),
            '+' => self.add_null_token("PLUS".to_string()),
            ';' => self.add_null_token("SEMICOLON".to_string()),
            '*' => self.add_null_token("STAR".to_string()),
            '=' => {
                let token_type: String = if self.operator_match('=') {
                    String::from("EQUAL_EQUAL")
                } else {
                    String::from("EQUAL")
                };

                self.add_null_token(token_type);
            }

            _ => { 
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, char);
                self.errors += 1;
            }
        }
    }

    fn add_null_token(&mut self, token_type: String) {
        self.add_token(token_type, "null".to_string());
    }

    fn add_token(&mut self, token_type: String, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push((token_type, text.to_string(), literal, self.line));
    }

    fn operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source.chars().nth(self.current + 2).unwrap_or( ' ' ) != expected { return false }
        self.current += 1;
        true
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

            if to_scan.errors > 0 { exit(65) }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

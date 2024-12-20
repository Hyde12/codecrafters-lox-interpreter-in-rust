use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use std::collections::HashMap;

// Scanner structure
struct Scanner {
    source: Vec<char>, // The source file
    tokens: Vec<(String, String, String, i32)>, // A vector of tuples (token_type, text, literal, line) that serve as our tokens
    start: usize, // Start of source
    current: usize, // Our current place in source
    line: i32, // Our current line in source
    errors: bool, // True if errors were present
    reserved_words: HashMap<&'static str, &'static str> // The keywords that the code uses
}

impl Scanner { 
    // Initialize a new Scanner 
    fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: false,
            reserved_words: HashMap::from([
                ("and", "AND"),
                ("class", "CLASS"),
                ("else", "ELSE"),
                ("true", "TRUE"),
                ("false", "FALSE"),
                ("for", "FOR"),
                ("while", "WHILE"),
                ("fun", "FUN"),
                ("if", "IF"),
                ("nil", "NIL"),
                ("or", "OR"),
                ("print", "PRINT"),
                ("return", "RETURN"),
                ("super", "SUPER"),
                ("this", "THIS"),
                ("var", "VAR"),
            ]),
        }
    }

    // Get all the tokens
    fn scan_tokens(&mut self) {
        // If not at end of the source then scan for more tokens
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token(self.current);
        }

        // End of file token when at the end
        self.tokens.push((String::from("EOF"), String::from(""), String::from("null"), self.line));
    }

    // Scan for token
    fn scan_token(&mut self, current: usize) {
        // Get the current character in the source
        let char = self.source[current];
        self.current += 1;

        match char {
            // If match, add token with null literal
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
            // If we find a pair of operators, change the type to match
            '=' => {         
                let token_type: String = if self.operator_match('=') {
                    String::from("EQUAL_EQUAL")
                } else {
                    String::from("EQUAL")
                };

                self.add_null_token(token_type);
            }
            '!' => {
                let token_type: String = if self.operator_match('=') {
                    String::from("BANG_EQUAL")
                } else {
                    String::from("BANG")
                };

                self.add_null_token(token_type);
            }
            '<' => {
                let token_type: String = if self.operator_match('=') {
                    String::from("LESS_EQUAL")
                } else {
                    String::from("LESS")
                };

                self.add_null_token(token_type);
            }
            '>' => {
                let token_type: String = if self.operator_match('=') {
                    String::from("GREATER_EQUAL")
                } else {
                    String::from("GREATER")
                };

                self.add_null_token(token_type);
            }
            '/' => { // As long as the next line isn't a new line, it's part of the comment
                if self.operator_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.current += 1;
                    }
          
                } else {
                    self.add_null_token(String::from("SLASH"));
                };

            }
            '"' => self.string(), // Get the string associated with the double quotes
            '\n' => self.line += 1, // New line
            '\t' | '\r' | ' ' => {} // Ignore
            _ => { 
                if char.is_numeric() { // If it's a number, add a number type 
                    self.number();
                } else if char.is_alphabetic() || char == '_' { // If it's alphabetic, add an identifier/keyword type 
                    self.identifier();
                } else { // If all else is false, it's not valid
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, char); 
                    self.errors = true;
                }
            }
        }
    }

    // Add a token with null literal
    fn add_null_token(&mut self, token_type: String) {
        self.add_token(token_type, "null".to_string());
    }

    // Pushes a token to the tokens vector from the start to the current place in the source
    fn add_token(&mut self, token_type: String, literal: String) {
        let text = self.source[self.start..self.current].iter().collect::<String>();

        self.tokens.push((token_type, text, literal, self.line));
    }

    // Checks the next character, returns true if matches with pair expectation
    fn operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source[self.current] != expected { return false }
        self.current += 1;

        true
    }

    // Adds a STRING type token if a corresponding end double quote is found, else return error
    fn string(&mut self) {
        while self.peek() != '\"' && !self.is_at_end() {
            self.current += 1;
        }

        if self.is_at_end() { 
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.errors = true;
            return;
        }

        self.current += 1;

        self.add_token(String::from("STRING"), self.source[self.start+1..self.current-1].iter().collect::<String>());
    }

    // Adds a NUMBER type with at least 1 floating point
    fn number(&mut self) {
        while self.peek().is_numeric() && !self.is_at_end() {
            self.current += 1;
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.current += 1;

            while self.peek().is_numeric() { self.current += 1 }
        }

        let value = self.source[self.start..self.current].iter().collect::<String>();
        let number = value.parse::<f32>().unwrap();

        if number == number.floor() {
            self.add_token(String::from("NUMBER"), number.to_string() + ".0");
        } else {
            self.add_token(String::from("NUMBER"), value.to_string());
        }
    }

    // Adds an IDENTIFIER type or RESERVED type based on the word taken
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.current += 1;
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type= self.reserved_words.get(text.as_str()).unwrap_or(&"IDENTIFIER");

        self.add_null_token(token_type.to_string());
    }

    // Returns true if at the end of source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Returns current character
    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.source[self.current]
    }

    // Returns next character
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0' }
        self.source[self.current + 1]
    }
}


fn main() {
    // Get the arguments from prompt
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

            // Initialize and declare a new scanner and scan for tokens
            let mut to_scan = Scanner::new(file_contents);
            to_scan.scan_tokens();

            // For every token from scan_tokens() we print it out
            for token in to_scan.tokens {
                let (token_type, text, literal, _line) = token;
                println!("{token_type} {text} {literal}");
            }

            // If we had any errors, exit with code 65
            if to_scan.errors { exit(65) }
        }
        _ => {
            // Print error if command was unkown
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
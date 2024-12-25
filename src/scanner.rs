use crate::tokens::TokenType;
use std::collections::HashMap;

pub struct Scanner {
    source: Vec<char>, // The source file
    pub tokens: Vec<(TokenType, String, String, i32)>, // A vector of tuples (token_type, text, literal, line) that serve as our tokens
    start: usize, // Start of source
    current: usize, // Our current place in source
    line: i32, // Our current line in source
    pub errors: bool, // True if errors were present
    reserved_words: HashMap<&'static str, TokenType>, // The keywords that the code uses
}

impl Scanner {
    // Initialize a new Scanner
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: false,
            reserved_words: HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("true", TokenType::TRUE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("while", TokenType::WHILE),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("var", TokenType::VAR),
            ]),
        }
    }

    // Get all the tokens
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push((TokenType::EOF, String::new(), String::from("null"), self.line));
    }

    // Scan for token
    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            '(' => self.add_null_token(TokenType::LEFT_PAREN),
            ')' => self.add_null_token(TokenType::RIGHT_PAREN),
            '{' => self.add_null_token(TokenType::LEFT_BRACE),
            '}' => self.add_null_token(TokenType::RIGHT_BRACE),
            ',' => self.add_null_token(TokenType::COMMA),
            '.' => self.add_null_token(TokenType::DOT),
            '-' => self.add_null_token(TokenType::MINUS),
            '+' => self.add_null_token(TokenType::PLUS),
            ';' => self.add_null_token(TokenType::SEMICOLON),
            '*' => self.add_null_token(TokenType::STAR),
            '=' => {
                let is_equal = self.operator_match('='); // Store the result in a local variable
                let token_type = if is_equal {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_null_token(token_type);
            }
            '!' => {
                let is_equal = self.operator_match('='); // Store the result in a local variable
                let token_type = if is_equal {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_null_token(token_type);
            }
            '<' => {
                let is_equal = self.operator_match('='); // Store the result in a local variable
                let token_type = if is_equal {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_null_token(token_type);
            }
            '>' => {
                let is_equal = self.operator_match('='); // Store the result in a local variable
                let token_type = if is_equal {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_null_token(token_type);
            }
            '/' => {
                if self.operator_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_null_token(TokenType::SLASH);
                }
            }
            '"' => self.string(),
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' => {}
            _ => {
                if char.is_numeric() {
                    self.number();
                } else if char.is_alphabetic() || char == '_' {
                    self.identifier();
                } else {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, char);
                    self.errors = true;
                }
            }
        }
    }

    fn add_null_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, "null".to_string());
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].iter().collect::<String>();
        self.tokens.push((token_type, text, literal, self.line));
    }

    fn operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.errors = true;
            return;
        }

        self.advance();
        self.add_token(TokenType::STRING, self.source[self.start + 1..self.current - 1].iter().collect());
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token(TokenType::NUMBER, value);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = self.reserved_words.get(text.as_str()).cloned().unwrap_or(TokenType::IDENTIFIER);
        self.add_null_token(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
}

use crate::lox_error::*;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    source: String,
    tokens: Vec<Token>,
    lexems: Vec<char>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source,
            tokens: Vec::new(),
            lexems: Vec::new(),
        }
    }

    /* SCANNER */

    // Entry point
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        self.lexems = self.source.chars().collect();

        while !self.is_end_file() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => LoxError::report(e, self.current),
            }
        }

        // Append 'EOF' token to list
        self.tokens
            .push(Token::new(String::new(), self.line, TokenType::Eof, None));

        return &self.tokens;
    }

    // Scan single char
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let l = self.advance();

        match l {
            '/' => {
                // Comments
                if self.is_matching('/') {
                    while self.peek() != '\n' && !self.is_end_file() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ';' => self.add_token(TokenType::Semicolon),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_matching('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.is_matching('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_matching('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_matching('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            // String literals
            '"' => {
                return match self.string() {
                    Ok(l) => Ok(self.add_token_object(TokenType::String, Some(l))),
                    Err(e) => Err(LoxError::error(self.line, e)),
                };
            }

            // Whitespace
            '\t' | '\r' | ' ' => {}

            // New line
            '\n' => self.line += 1,

            default => {
                // Digits
                if default.is_digit(10) {
                    let digit = self.number();
                    self.add_token_object(TokenType::Number, Some(digit))
                } else {
                    // Error
                    return Err(LoxError::error(
                        self.line,
                        "Unexpected Character".to_string(),
                    ));
                }
            }
        };
        Ok(())
    }

    /* HELPERS */

    fn number(&mut self) -> String {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Checks if their is a dot and a number after the current
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        return self.source[self.start..self.current].to_string();
    }

    // Handles string literals
    fn string(&mut self) -> Result<String, String> {
        while self.peek() != '"' && !self.is_end_file() {
            if self.peek() == '\n' {
                self.line += 1;
            };
            self.advance();
        }
        if self.is_end_file() {
            return Err("Undeterminated string".to_string());
        }

        // Get the closing "
        self.advance();

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        Ok(value)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.lexems[self.current + 1];
    }

    // Peeks to the next character without consuming it
    fn peek(&self) -> char {
        if self.is_end_file() {
            return '\0';
        }
        return self.lexems[self.current];
    }

    // Advance and consume the character
    fn advance(&mut self) -> char {
        let current_char = self.lexems[self.current];
        self.current += 1;
        return current_char;
    }

    fn is_end_file(&self) -> bool {
        return self.current >= self.source.len();
    }

    // Matches with the next character if found and consumes it
    fn is_matching(&mut self, expected: char) -> bool {
        if self.is_end_file() {
            return false;
        }
        let res = self.lexems[self.current];

        if res == expected {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }

    /* FINAL */

    fn add_token(&mut self, token: TokenType) {
        self.add_token_object(token, None);
    }

    fn add_token_object(&mut self, token: TokenType, object: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(text, self.line, token, object);
        self.tokens.push(token);
    }
}

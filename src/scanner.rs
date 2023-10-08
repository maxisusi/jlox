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

    /* ## SCANNER ## */

    // -> Entry point
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

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let l = self.advance_pointer();

        match l {
            '/' => {
                // Comments (ignored)
                if self.is_matching('/') {
                    while self.peek() != '\n' && !self.is_end_file() {
                        self.advance_pointer();
                    }

                // Single slash
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
                // Bang equal
                if self.is_matching('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                // Equal equal
                if self.is_matching('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                // Less equal
                if self.is_matching('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                // Greater equal
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

            // Whitespaces (ignored)
            '\t' | '\r' | ' ' => {}

            // New line
            '\n' => self.line += 1,

            default => {
                // Numbers and floating numbers
                if default.is_digit(10) {
                    self.number(|s, digit| s.add_token_object(TokenType::Number, Some(digit)));

                // Identifiers and keywords
                } else if default.is_alphabetic() {
                    self.identifier(|s, token_type| s.add_token(token_type));

                // Unexpected character
                } else {
                    return Err(LoxError::error(
                        self.line,
                        "Unexpected Character".to_string(),
                    ));
                }
            }
        };
        Ok(())
    }

    /* ## HELPERS ##  */

    // Handles identifiers and keywords
    fn identifier<F>(&mut self, callback: F)
    where
        F: Fn(&mut Self, TokenType),
    {
        while self.peek().is_alphanumeric() {
            self.advance_pointer();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier).clone();
        callback(self, token_type);
    }

    // Handles numbers and floating numbers
    fn number<F>(&mut self, callback: F)
    where
        F: Fn(&mut Self, String),
    {
        while self.peek().is_digit(10) {
            self.advance_pointer();
        }

        // Checks if their is a dot and a number after the current
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance_pointer();

            while self.peek().is_digit(10) {
                self.advance_pointer();
            }
        }
        callback(self, self.source[self.start..self.current].to_string())
    }

    // Handles string literals
    fn string(&mut self) -> Result<String, String> {
        while self.peek() != '"' && !self.is_end_file() {
            if self.peek() == '\n' {
                self.line += 1;
            };
            self.advance_pointer();
        }
        if self.is_end_file() {
            return Err("Undeterminated string".to_string());
        }

        // Get the closing "
        self.advance_pointer();

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        Ok(value)
    }

    // Peeks to the next character without moving the pointer
    fn peek(&self) -> char {
        if self.is_end_file() {
            return '\0';
        }
        return self.lexems[self.current];
    }

    // Peek one position after the peek
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.lexems[self.current + 1];
    }

    // Advance the pointer the character
    fn advance_pointer(&mut self) -> char {
        let current_char = self.lexems[self.current];
        self.current += 1;
        return current_char;
    }

    // Check if pointer is at the end of file
    fn is_end_file(&self) -> bool {
        return self.current >= self.source.len();
    }

    // Match a character and advence the pointer
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

    /* ## TOKEN GENERATOR ## */

    fn add_token(&mut self, token: TokenType) {
        self.add_token_object(token, None);
    }

    fn add_token_object(&mut self, token: TokenType, object: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(text, self.line, token, object);
        self.tokens.push(token);
    }
}

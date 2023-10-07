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

    // Entry point for scanner
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
            .push(Token::new(String::new(), self.line, TokenType::Eof));

        return &self.tokens;
    }

    // Scan single token
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let l = self.advance();

        match l {
            // Tokens
            '/' => self.add_token(TokenType::Slash),
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

            // Ignore whitespace
            '\t' | '\r' | ' ' => {}

            // New line
            '\n' => self.line += 1,

            // Error
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected Character".to_string(),
                ))
            }
        };
        Ok(())
    }

    fn advance(&mut self) -> char {
        let res = self.lexems[self.current];
        self.current += 1;
        return res;
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_object(token);
    }

    fn add_token_object(&mut self, token: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(text, self.line, token);
        self.tokens.push(token);
    }

    fn is_end_file(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn is_matching(&mut self, expected: char) -> bool {
        if self.is_end_file() {
            return false;
        }
        return match self.source.chars().nth(self.current) {
            Some(e) => {
                if e == expected {
                    self.current += 1;
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        };
    }
}

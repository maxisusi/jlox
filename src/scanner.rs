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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // Transform source into list of chars
        self.lexems = self.source.chars().collect();
        while !self.is_end_file() {
            self.start = self.current;
            self.scan_token();
        }

        // Append EOF token to list
        self.tokens
            .push(Token::new(String::new(), self.line, TokenType::Eof));

        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let l = self.advance();

        match l {
            // Tokens
            '/' => self.define_t(TokenType::Slash),
            '(' => self.define_t(TokenType::LeftParen),
            ')' => self.define_t(TokenType::RightParen),
            '{' => self.define_t(TokenType::LeftBrace),
            '}' => self.define_t(TokenType::RightBrace),
            ';' => self.define_t(TokenType::Semicolon),
            ',' => self.define_t(TokenType::Comma),
            '.' => self.define_t(TokenType::Dot),
            '-' => self.define_t(TokenType::Minus),
            '+' => self.define_t(TokenType::Plus),
            '*' => self.define_t(TokenType::Star),
            '!' => self.define_t(TokenType::Bang),
            '=' => self.define_t(TokenType::Equal),
            '<' => self.define_t(TokenType::Less),
            '>' => self.define_t(TokenType::Greater),
            // Ignore whitespace
            '\t' | '\r' | ' ' => {}
            // New line
            '\n' => self.line += 1,
            // Error
            _ => {
                return println!("Unrecognized token: '{}' at line: '{}'", l, self.line);
            }
        };
    }

    fn advance(&mut self) -> char {
        let res = self.lexems[self.current];
        self.current += 1;
        return res;
    }

    fn define_t(&mut self, token: TokenType) {
        self.add_t(token);
    }

    fn add_t(&mut self, token: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(text, self.line, token);
        self.tokens.push(token);
    }

    fn is_end_file(&self) -> bool {
        return self.current >= self.source.len();
    }
}

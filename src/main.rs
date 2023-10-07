use std::fmt::Debug;
use std::fs;
use std::io::{self};
use std::{env, process::exit}; // Import the Write trait

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    RightParen,
    LeftParen,
    RightBrace,
    LeftBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    // BANG_EQUAL,
    Equal,
    // EQUAL_EQUAL,
    Greater,
    // GREATER_EQUAL,
    Less,
    // LESS_EQUAL,

    // Literals
    Eof,
}

struct Token {
    lexeme: String,
    line: usize,
    token_type: TokenType,
}

impl Token {
    fn new(lexeme: String, line: usize, token_type: TokenType) -> Token {
        Token {
            lexeme,
            line,
            token_type,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.line)
    }
}

struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    source: String,
    tokens: Vec<Token>,
    lexems: Vec<char>,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source,
            tokens: Vec::new(),
            lexems: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
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
        return self.current >= self.source.len() - 1;
    }
}

fn main() {
    // Collect the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        // Run the file
        let path = &args[1];
        let file = fs::read_to_string(path).expect("Could not read the file");

        // Feed the file inside the scanner
        let mut scanner = Scanner::new(file);

        // Get the tokens list
        let tokens = scanner.scan_tokens();

        // Print the tokens
        for token in tokens {
            println!("{}, {:?}, {}", token.lexeme, token.token_type, token.line);
        }
    } else {
        // Run interactively
        run_prompt();
    }
}

fn run_prompt() {
    loop {
        let mut prompt_input = String::new();
        io::stdin().read_line(&mut prompt_input).unwrap();

        if prompt_input.contains("exit()") {
            exit(0);
        }
        println!("You typed: {}", prompt_input);
    }
}

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
    line: u32,
    token_type: TokenType,
}

impl Token {
    fn new(lexeme: String, line: u32, token_type: TokenType) -> Token {
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
    start: u32,
    current: u32,
    line: u32,
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source,
            tokens: Vec::new(),
        }
    }

    fn scan_token(&mut self) {
        let file = self.source.chars();
        for (idx, t) in file.enumerate() {
            self.current = idx as u32;

            match t {
                '/' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Slash)),
                '(' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::LeftParen)),
                ')' => {
                    self.tokens
                        .push(self.add_t(t.to_string(), self.line, TokenType::RightParen))
                }
                '{' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::LeftBrace)),
                '}' => {
                    self.tokens
                        .push(self.add_t(t.to_string(), self.line, TokenType::RightBrace))
                }
                ';' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Semicolon)),
                ',' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Comma)),
                '.' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Dot)),
                '-' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Minus)),
                '+' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Plus)),
                '*' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Star)),
                '!' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Bang)),
                '=' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Equal)),
                '<' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Less)),
                '>' => self
                    .tokens
                    .push(self.add_t(t.to_string(), self.line, TokenType::Greater)),
                ' ' => {}
                '\n' => self.line += 1,
                _ => {
                    println!("Unrecognized token: '{}' at line: '{}'", t, self.line);
                }
            };
        }
        // Adding the EOF token
        self.tokens
            .push(Token::new(String::from(""), self.line, TokenType::Eof));
    }

    fn add_t(&self, lexeme: String, line: u32, token_type: TokenType) -> Token {
        return Token::new(lexeme, line, token_type);
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
        scanner.scan_token();

        // Print the tokens
        for token in scanner.tokens {
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

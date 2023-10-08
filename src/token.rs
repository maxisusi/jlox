use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub line: usize,
    pub token_type: TokenType,
    pub object: Option<String>,
}

impl Token {
    pub fn new(
        lexeme: String,
        line: usize,
        token_type: TokenType,
        object: Option<String>,
    ) -> Token {
        Token {
            lexeme,
            line,
            token_type,
            object,
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

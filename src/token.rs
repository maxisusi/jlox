use crate::token_type::TokenType;

#[derive(Debug)]
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

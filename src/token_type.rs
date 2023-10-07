#[derive(Debug)]
pub enum TokenType {
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
    // Literals
    Eof,
}

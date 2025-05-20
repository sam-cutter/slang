use crate::source::Location;

#[derive(Debug)]
pub struct Token {
    category: TokenCategory,
    start: Location,
    length: usize,
}

impl Token {
    pub fn new(category: TokenCategory, start: Location, length: usize) -> Self {
        Self {
            category,
            start,
            length,
        }
    }
}

#[derive(Debug)]
pub enum TokenCategory {
    // Single character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,

    // Logical comparisons
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    DoubleAmpersand,
    DoublePipe,

    // Control flow
    If,
    Else,
    While,

    Identifier(String),

    // Raw values
    String(String),
    Number(f64),
    Boolean(bool),

    Fun,
    Return,
    Let,
}

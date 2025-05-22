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
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,

    // Logical and bitwise operators
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Ampersand,
    DoubleAmpersand,
    Pipe,
    DoublePipe,

    // Literals
    String(String),
    Number(f64),
    Boolean(bool),
    Null,

    // Control flow
    If,
    Else,
    While,
    Return,

    // Identifier related
    Let,
    Fun,
    Identifier(String),
}

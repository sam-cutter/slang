use crate::source::Location;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    start: Location,
    length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: Location, length: usize) -> Self {
        Self {
            kind,
            start,
            length,
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
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
    Fu,
    Identifier(String),
}

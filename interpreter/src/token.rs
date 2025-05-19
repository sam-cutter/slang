use crate::source::Location;

#[derive(Debug)]
pub struct Token {
    category: TokenCategory,
    start: Location,
    end: Location,
}

impl Token {
    pub fn new(category: TokenCategory, start: Location, end: Location) -> Self {
        Self {
            category,
            start,
            end,
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
    String(String),
    Number(f64),

    // Booleans
    True,
    False,

    Fun,
    Return,
    Let,
}

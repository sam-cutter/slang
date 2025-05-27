use crate::source::Location;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    start: Location,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, start: Location) -> Self {
        Self {
            kind,
            lexeme,
            start,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    String,
    Number,
    Boolean,
    Null,

    // Control flow
    If,
    Else,
    While,
    Return,

    // Identifier related
    Let,
    Fu,
    Identifier,
}

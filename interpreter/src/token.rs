use std::mem;

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

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn has_kind(&self, kind: &TokenKind) -> bool {
        mem::discriminant(&self.kind) == mem::discriminant(kind)
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

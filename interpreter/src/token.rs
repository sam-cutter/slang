use crate::source::Location;

#[derive(Debug)]
pub struct Token {
    data: TokenData,
    start: Location,
    length: usize,
}

impl Token {
    pub fn new(data: TokenData, start: Location, length: usize) -> Self {
        Self {
            data,
            start,
            length,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.data.kind()
    }

    pub fn start(&self) -> Location {
        self.start
    }

    pub fn data(self) -> TokenData {
        self.data
    }
}

#[derive(Debug)]
pub enum TokenData {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    QuestionMark,
    Colon,

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

impl TokenData {
    pub fn kind(&self) -> TokenKind {
        match self {
            TokenData::LeftParenthesis => TokenKind::LeftParenthesis,
            TokenData::RightParenthesis => TokenKind::RightParenthesis,
            TokenData::LeftBrace => TokenKind::LeftBrace,
            TokenData::RightBrace => TokenKind::RightBrace,
            TokenData::Comma => TokenKind::Comma,
            TokenData::Dot => TokenKind::Dot,
            TokenData::Semicolon => TokenKind::Semicolon,
            TokenData::QuestionMark => TokenKind::QuestionMark,
            TokenData::Colon => TokenKind::Colon,

            // Arithmetic operators
            TokenData::Plus => TokenKind::Plus,
            TokenData::Minus => TokenKind::Minus,
            TokenData::Star => TokenKind::Star,
            TokenData::Slash => TokenKind::Slash,

            // Logical and bitwise operators
            TokenData::Bang => TokenKind::Bang,
            TokenData::BangEqual => TokenKind::BangEqual,
            TokenData::Equal => TokenKind::Equal,
            TokenData::DoubleEqual => TokenKind::DoubleEqual,
            TokenData::Greater => TokenKind::Greater,
            TokenData::GreaterEqual => TokenKind::GreaterEqual,
            TokenData::Less => TokenKind::Less,
            TokenData::LessEqual => TokenKind::LessEqual,
            TokenData::Ampersand => TokenKind::Ampersand,
            TokenData::DoubleAmpersand => TokenKind::DoubleAmpersand,
            TokenData::Pipe => TokenKind::Pipe,
            TokenData::DoublePipe => TokenKind::DoublePipe,

            // Literals
            TokenData::String(_) => TokenKind::String,
            TokenData::Number(_) => TokenKind::Number,
            TokenData::Boolean(_) => TokenKind::Boolean,
            TokenData::Null => TokenKind::Null,

            // Control flow
            TokenData::If => TokenKind::If,
            TokenData::Else => TokenKind::Else,
            TokenData::While => TokenKind::While,
            TokenData::Return => TokenKind::Return,

            // Identifier related
            TokenData::Let => TokenKind::Let,
            TokenData::Fu => TokenKind::Fu,
            TokenData::Identifier(_) => TokenKind::Identifier,
        }
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
    QuestionMark,
    Colon,

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

impl TokenData {
    pub fn raw(&self) -> String {
        match self {
            TokenData::LeftParenthesis => "(",
            TokenData::RightParenthesis => ")",
            TokenData::LeftBrace => "{",
            TokenData::RightBrace => "}",
            TokenData::Comma => ",",
            TokenData::Dot => ".",
            TokenData::Semicolon => ";",
            TokenData::QuestionMark => "?",
            TokenData::Colon => ":",

            TokenData::Plus => "+",
            TokenData::Minus => "-",
            TokenData::Star => "*",
            TokenData::Slash => "/",

            TokenData::Bang => "!",
            TokenData::BangEqual => "!=",
            TokenData::Equal => "=",
            TokenData::DoubleEqual => "==",
            TokenData::Greater => ">",
            TokenData::GreaterEqual => ">=",
            TokenData::Less => "<",
            TokenData::LessEqual => "<=",
            TokenData::Ampersand => "&",
            TokenData::DoubleAmpersand => "&&",
            TokenData::Pipe => "|",
            TokenData::DoublePipe => "||",

            TokenData::Null => "null",
            TokenData::If => "if",
            TokenData::Else => "else",
            TokenData::While => "while",
            TokenData::Return => "return",
            TokenData::Let => "let",
            TokenData::Fu => "fu",

            // For variants with data, handle separately
            TokenData::String(string) => return format!("\"{}\"", string),
            TokenData::Number(number) => return number.to_string(),
            TokenData::Boolean(boolean) => return boolean.to_string(),
            TokenData::Identifier(identifier) => return identifier.clone(),
        }
        .to_string()
    }
}

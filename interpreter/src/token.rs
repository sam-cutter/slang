use crate::{
    expression::{BinaryOperator, UnaryOperator},
    source::Location,
};

#[derive(Debug)]
pub struct Token {
    data: TokenData,
    location: Location,
    length: usize,
}

impl Token {
    pub fn new(data: TokenData, location: Location, length: usize) -> Self {
        Self {
            data,
            location,
            length,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.data.kind()
    }

    pub fn location(&self) -> Location {
        self.location
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
    Float(f64),
    Integer(i32),
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

    Print,
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
            TokenData::Float(_) => TokenKind::Float,
            TokenData::Integer(_) => TokenKind::Integer,
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

            TokenData::Print => TokenKind::Print,
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
    Float,
    Integer,
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

    Print,
}

impl TokenKind {
    pub fn binary_operator(&self) -> Option<BinaryOperator> {
        Some(match self {
            Self::Plus => BinaryOperator::Add,
            Self::Minus => BinaryOperator::Subtract,
            Self::Star => BinaryOperator::Multiply,
            Self::Slash => BinaryOperator::Divide,

            Self::DoubleEqual => BinaryOperator::EqualTo,
            Self::BangEqual => BinaryOperator::NotEqualTo,
            Self::Greater => BinaryOperator::GreaterThan,
            Self::GreaterEqual => BinaryOperator::GreaterThanOrEqualTo,
            Self::Less => BinaryOperator::LessThan,
            Self::LessEqual => BinaryOperator::LessThanOrEqualTo,

            Self::Ampersand => BinaryOperator::BitwiseAND,
            Self::DoubleAmpersand => BinaryOperator::AND,
            Self::Pipe => BinaryOperator::BitwiseOR,
            Self::DoublePipe => BinaryOperator::OR,

            _ => return None,
        })
    }

    pub fn unary_operator(&self) -> Option<UnaryOperator> {
        Some(match self {
            Self::Minus => UnaryOperator::Minus,
            Self::Bang => UnaryOperator::NOT,

            _ => return None,
        })
    }
}

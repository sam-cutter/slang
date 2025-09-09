//! Tokens within the slang programming language.

use crate::{
    expression::{BinaryOperator, UnaryOperator},
    source::Location,
};

/// The smallest meaningful unit of the language.
#[derive(Debug)]
pub struct Token {
    /// The contained data, including the token type, and any associated data.
    data: TokenData,
    /// The location of its first character.
    location: Location,
}

impl Token {
    /// Creates a new [Token].
    pub fn new(data: TokenData, location: Location) -> Self {
        Self { data, location }
    }

    /// Returns the kind of the token.
    pub fn kind(&self) -> TokenKind {
        self.data.kind()
    }

    /// Returns the location of the token's first character.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Consumes the token and returns its data.
    pub fn data(self) -> TokenData {
        self.data
    }
}

/// The data contained within a token.
///
/// This is similar to [TokenKind], however contains more information. For example, the [TokenData::Integer] variant has an [i32] field which stores the integer that token represents, however [TokenKind::Integer] has no contained fields, and is simply a flag stating that the token represents an integer.
#[derive(Debug)]
pub enum TokenData {
    /// The `(` character.
    LeftParenthesis,
    /// The `)` character.
    RightParenthesis,
    /// The `{` character.
    LeftBrace,
    /// The `}` character.
    RightBrace,
    /// The `,` character.
    Comma,
    /// The `.` character.
    Dot,
    /// The `;` character.
    Semicolon,
    /// The `?` character.
    QuestionMark,
    /// The `:` character.
    Colon,

    // Arithmetic operators
    /// The `+` character.
    Plus,
    /// The `-` character.
    Minus,
    /// The `*` character.
    Star,
    /// The `/` character.
    Slash,
    /// The `^` character.
    Exponent,

    // Logical and bitwise operators
    /// The `!` character.
    Bang,
    /// The `!=` string.
    BangEqual,
    /// The `=` character.
    Equal,
    /// The `==` string.
    DoubleEqual,
    /// The `>` character.
    Greater,
    /// The `>=` string.
    GreaterEqual,
    /// The `<` character.
    Less,
    /// The `<=` string.
    LessEqual,
    /// The `&` character.
    Ampersand,
    /// The `&&` string.
    DoubleAmpersand,
    /// The `|` character.
    Pipe,
    /// The `||` string.
    DoublePipe,

    // Literals
    /// String literals enclosed in `"`.
    String(String),
    /// Floating point numbers, denoted with a `.` separating the integer and fractional parts.
    Float(f64),
    /// Integers.
    Integer(i32),
    /// Either `true` or `false`.
    Boolean(bool),

    // Control flow
    /// The `if` string.
    If,
    /// The `else` string.
    Else,
    /// The `while` string.
    While,
    /// The `return` string.
    Return,

    // Identifier related
    /// The `let` string.
    Let,
    /// The `fu` string.
    Fu,
    /// All valid identifiers.
    ///
    /// Must start with either an alphabetic character or an underscore, with all subsequent characters being alphanumeric or underscores.
    Identifier(String),
}

impl TokenData {
    /// Returns the [TokenKind] of some [TokenData].
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
            TokenData::Exponent => TokenKind::Exponent,

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

/// A flag signalling the type of a token, without any additional data.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    /// The `(` character.
    LeftParenthesis,
    /// The `)` character.
    RightParenthesis,
    /// The `{` character.
    LeftBrace,
    /// The `}` character.
    RightBrace,
    /// The `,` character.
    Comma,
    /// The `.` character.
    Dot,
    /// The `;` character.
    Semicolon,
    /// The `?` character.
    QuestionMark,
    /// The `:` character.
    Colon,

    // Arithmetic operators
    /// The `+` character.
    Plus,
    /// The `-` character.
    Minus,
    /// The `*` character.
    Star,
    /// The `/` character.
    Slash,
    /// The `^` character.
    Exponent,

    // Logical and bitwise operators
    /// The `!` character.
    Bang,
    /// The `!=` string.
    BangEqual,
    /// The `=` character.
    Equal,
    /// The `==` string.
    DoubleEqual,
    /// The `>` character.
    Greater,
    /// The `>=` string.
    GreaterEqual,
    /// The `<` character.
    Less,
    /// The `<=` string.
    LessEqual,
    /// The `&` character.
    Ampersand,
    /// The `&&` string.
    DoubleAmpersand,
    /// The `|` character.
    Pipe,
    /// The `||` string.
    DoublePipe,

    // Literals
    /// String literals enclosed in `"`.
    String,
    /// Floating point numbers, denoted with a `.` separating the integer and fractional parts.
    Float,
    /// Integers.
    Integer,
    /// Either `true` or `false`.
    Boolean,

    // Control flow
    /// The `if` string.
    If,
    /// The `else` string.
    Else,
    /// The `while` string.
    While,
    /// The `return` string.
    Return,

    // Identifier related
    /// The `let` string.
    Let,
    /// The `fu` string.
    Fu,
    /// All valid identifiers.
    ///
    /// Must start with either an alphabetic character or an underscore, with all subsequent characters being alphanumeric or underscores.
    Identifier,
}

impl TokenKind {
    /// Attempts to cast itself to a [BinaryOperator], returning [None] if it does not represent a binary operator.
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

    /// Attempts to cast itself to a [UnaryOperator], returning [None] if it does not represent a unary operator.
    pub fn unary_operator(&self) -> Option<UnaryOperator> {
        Some(match self {
            Self::Minus => UnaryOperator::Minus,
            Self::Bang => UnaryOperator::NOT,

            _ => return None,
        })
    }
}

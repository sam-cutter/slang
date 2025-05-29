use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    expression::{Expression, Literal},
    source::GeneralLocation,
    token::{TokenData, TokenKind},
    token_stream::TokenStream,
};

pub enum ParserError {
    ExpectedToken {
        expected: Vec<TokenKind>,
        location: GeneralLocation,
    },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedToken { expected, location } => {
                write!(
                    f,
                    "{} Expected one of the following tokens: {:?}",
                    location, expected
                )
            }
        }
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ParserError {}

pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens }
    }

    pub fn parse(mut self) -> Result<Vec<Expression>, Vec<ParserError>> {
        let mut expressions: Vec<Expression> = Vec::new();
        let mut errors: Vec<ParserError> = Vec::new();

        while self.tokens.peek().is_some() {
            match self.expression() {
                Ok(expression) => expressions.push(expression),
                Err(error) => {
                    errors.push(error);
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(expressions)
        } else {
            Err(errors)
        }
    }

    fn synchronize(&mut self) {
        self.tokens.advance();

        while let Some(token) = self.tokens.peek() {
            match token.kind() {
                TokenKind::Semicolon => {
                    self.tokens.advance();
                    return;
                }

                TokenKind::Fu
                | TokenKind::Let
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Return => return,

                _ => {
                    self.tokens.advance();
                }
            }
        }
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.comparison()?;

        while let Some(operator) = self
            .tokens
            .matches(&[TokenKind::BangEqual, TokenKind::DoubleEqual])
        {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.comparison()?),
            }
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.term()?;

        while let Some(operator) = self.tokens.matches(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.term()?),
            }
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.factor()?;

        while let Some(operator) = self.tokens.matches(&[TokenKind::Plus, TokenKind::Minus]) {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.factor()?),
            }
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.unary()?;

        while let Some(operator) = self.tokens.matches(&[TokenKind::Star, TokenKind::Slash]) {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.unary()?),
            }
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if let Some(operator) = self.tokens.matches(&[TokenKind::Bang, TokenKind::Minus]) {
            Ok(Expression::Unary {
                operator: operator,
                operand: Box::new(self.primary()?),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let expected = [
            TokenKind::LeftParenthesis,
            TokenKind::String,
            TokenKind::Number,
            TokenKind::Boolean,
            TokenKind::Null,
        ];

        if let Some(token) = self.tokens.matches(&expected) {
            Ok(match token.data() {
                TokenData::LeftParenthesis => {
                    self.tokens.advance();

                    let expression = self.expression()?;

                    self.tokens.consume(TokenKind::RightParenthesis)?;

                    Expression::Grouping(Box::new(expression))
                }

                TokenData::String(string) => Expression::Literal(Literal::String(string)),

                TokenData::Number(number) => Expression::Literal(Literal::Number(number)),

                TokenData::Boolean(boolean) => Expression::Literal(Literal::Boolean(boolean)),

                TokenData::Null => Expression::Literal(Literal::Null),

                _ => unreachable!(),
            })
        } else if let Some(token) = self.tokens.peek() {
            Err(ParserError::ExpectedToken {
                expected: expected.to_vec(),
                location: GeneralLocation::Location(token.start()),
            })
        } else {
            Err(ParserError::ExpectedToken {
                expected: expected.to_vec(),
                location: GeneralLocation::EndOfFile,
            })
        }
    }
}

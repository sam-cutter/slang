use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    expression::{Expression, Literal},
    token::TokenKind,
    token_stream::TokenStream,
};

// TODO: figure out how to store location information here
pub enum ParserError {
    ExpectedToken(Vec<TokenKind>),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedToken(expected) => {
                write!(f, "Expected one of the following tokens: {:?}", expected)
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
        Self { tokens: tokens }
    }

    pub fn expression(&mut self) -> Result<Expression, ParserError> {
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
        let expected_error = ParserError::ExpectedToken(Vec::from([
            TokenKind::Boolean,
            TokenKind::Null,
            TokenKind::Number,
            TokenKind::String,
        ]));

        let result = if let Some(token) = self.tokens.peek() {
            Ok(match token.kind() {
                TokenKind::LeftParenthesis => {
                    self.tokens.advance();

                    let expression = self.expression()?;

                    self.tokens.consume(TokenKind::RightParenthesis)?;

                    Expression::Grouping(Box::new(expression))
                }

                TokenKind::String => Expression::Literal(Literal::String(
                    token
                        .lexeme()
                        .get(1..token.lexeme().len() - 2)
                        .unwrap()
                        .to_string(),
                )),

                TokenKind::Number => {
                    Expression::Literal(Literal::Number(token.lexeme().parse().unwrap()))
                }

                TokenKind::Boolean => {
                    Expression::Literal(Literal::Boolean(token.lexeme().parse().unwrap()))
                }

                TokenKind::Null => Expression::Literal(Literal::Null),

                _ => Err(expected_error)?,
            })
        } else {
            Err(expected_error)
        };

        if result.is_ok() {
            self.tokens.advance();
        }

        result
    }
}

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    expression::{BinaryOperator, Expression, Literal, UnaryOperator},
    source::GeneralLocation,
    statement::Statement,
    token::{TokenData, TokenKind},
    token_stream::TokenStream,
};

pub enum ParserError {
    ExpectedToken {
        expected: Vec<TokenKind>,
        location: GeneralLocation,
    },
    UnsupportedUnaryExpression {
        operator: BinaryOperator,
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
            Self::UnsupportedUnaryExpression { operator, location } => {
                write!(
                    f,
                    "{} The unary `{}` operator is not supported.",
                    location,
                    operator.raw(),
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

    pub fn parse(mut self) -> Result<Vec<Statement>, Vec<ParserError>> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<ParserError> = Vec::new();

        while self.tokens.peek().is_some() {
            match self.statement() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    errors.push(error);
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
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

    fn statement(&mut self) -> Result<Statement, ParserError> {
        // Handle expression statement
        // Handle print statement
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.logical()?;

        if self.tokens.matches(&[TokenKind::QuestionMark]).is_some() {
            let left = self.logical()?;

            self.tokens.consume(TokenKind::Colon)?;

            let right = self.logical()?;

            expression = Expression::Ternary {
                condition: Box::new(expression),
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn logical(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.equality()?;

        while let Some((operator, _)) = self
            .tokens
            .binary_operator(&[BinaryOperator::AND, BinaryOperator::OR])
        {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(self.equality()?),
            }
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.comparison()?;

        while let Some((operator, _)) = self
            .tokens
            .binary_operator(&[BinaryOperator::NotEqualTo, BinaryOperator::EqualTo])
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
        let mut expression = self.bitwise()?;

        while let Some((operator, _)) = self.tokens.binary_operator(&[
            BinaryOperator::GreaterThan,
            BinaryOperator::GreaterThanOrEqualTo,
            BinaryOperator::LessThan,
            BinaryOperator::LessThanOrEqualTo,
        ]) {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.bitwise()?),
            }
        }

        Ok(expression)
    }

    fn bitwise(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.term()?;

        while let Some((operator, _)) = self
            .tokens
            .binary_operator(&[BinaryOperator::BitwiseAND, BinaryOperator::BitwiseOR])
        {
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

        while let Some((operator, _)) = self
            .tokens
            .binary_operator(&[BinaryOperator::Add, BinaryOperator::Subtract])
        {
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

        while let Some((operator, _)) = self
            .tokens
            .binary_operator(&[BinaryOperator::Multiply, BinaryOperator::Divide])
        {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.unary()?),
            }
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if let Some((operator, _)) = self
            .tokens
            .unary_operator(&[UnaryOperator::Minus, UnaryOperator::NOT])
        {
            Ok(Expression::Unary {
                operator: operator,
                operand: Box::new(self.primary()?),
            })
        } else if let Some((operator, location)) = self.tokens.binary_operator(&[
            BinaryOperator::Add,
            BinaryOperator::Multiply,
            BinaryOperator::Divide,
            BinaryOperator::NotEqualTo,
            BinaryOperator::EqualTo,
            BinaryOperator::GreaterThan,
            BinaryOperator::GreaterThanOrEqualTo,
            BinaryOperator::LessThan,
            BinaryOperator::LessThanOrEqualTo,
            BinaryOperator::BitwiseAND,
            BinaryOperator::BitwiseOR,
        ]) {
            let _ = self.primary();

            Err(ParserError::UnsupportedUnaryExpression {
                location: GeneralLocation::Location(location),
                operator: operator,
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let expected = [
            TokenKind::LeftParenthesis,
            TokenKind::String,
            TokenKind::Float,
            TokenKind::Integer,
            TokenKind::Boolean,
            TokenKind::Null,
        ];

        if let Some(token) = self.tokens.matches(&expected) {
            Ok(Expression::Literal(match token.data() {
                TokenData::LeftParenthesis => {
                    let expression = self.expression()?;

                    self.tokens.consume(TokenKind::RightParenthesis)?;

                    return Ok(Expression::Grouping(Box::new(expression)));
                }

                TokenData::String(string) => Literal::String(string),

                TokenData::Float(float) => Literal::Float(float),

                TokenData::Integer(integer) => Literal::Integer(integer),

                TokenData::Boolean(boolean) => Literal::Boolean(boolean),

                TokenData::Null => Literal::Null,

                _ => unreachable!(),
            }))
        } else if let Some(token) = self.tokens.peek() {
            Err(ParserError::ExpectedToken {
                expected: expected.to_vec(),
                location: GeneralLocation::Location(token.location()),
            })
        } else {
            Err(ParserError::ExpectedToken {
                expected: expected.to_vec(),
                location: GeneralLocation::EndOfFile,
            })
        }
    }
}

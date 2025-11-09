//! The parser for the slang programming language.

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    expression::{BinaryOperator, Expression, UnaryOperator},
    source::{GeneralLocation, Location},
    statement::Statement,
    token::{TokenData, TokenKind},
    token_stream::TokenStream,
    value::Value,
};

/// All errors which can occur while parsing.
pub enum ParserError {
    /// When a token was expected but not found.
    ExpectedToken {
        expected: Vec<TokenKind>,
        location: GeneralLocation,
    },
    /// When a unary expression with an unsupported unary operator is encountered.
    UnsupportedUnaryExpression {
        operator: BinaryOperator,
        location: GeneralLocation,
    },
    /// When there is an attempt to assign a value to something which is not assignable.
    InvalidAssignmentTarget(Location),
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
            Self::InvalidAssignmentTarget(location) => {
                write!(f, "{} Invalid assignment target.", location)
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

/// A parser for a specific token stream.
pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    /// Creates a new parser for a specific token stream.
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens }
    }

    /// Attempts to parse the token stream. Corresponds to `program` in the grammar.
    ///
    /// Consumes the entire token stream. Will attempt to find all errors, while minimising cascading errors.
    pub fn parse(mut self) -> Result<Vec<Statement>, Vec<ParserError>> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<ParserError> = Vec::new();

        while !self.tokens.at_end() {
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

    /// Consumes tokens until the end of a statement is reached.
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

    /// Attempts to parse a statement. Corresponds to `statement` in the grammar.
    fn statement(&mut self) -> Result<Statement, ParserError> {
        match self.tokens.peek().map(|token| token.kind()) {
            Some(TokenKind::Let) => self.variable_declaration(),
            Some(TokenKind::Fu) => self.function_definition(),
            Some(TokenKind::Return) => self.return_statement(),
            Some(TokenKind::If) => self.if_statement(),
            Some(TokenKind::While) => self.while_loop(),
            Some(TokenKind::LeftBrace) => self.block(),
            _ => self.expression_statement(),
        }
    }

    /// Attempts to parse a variable declaration. Corresponds to `variableDeclaration` in the grammar.
    fn variable_declaration(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::Let)?;

        let identifier = self.tokens.consume_identifier()?;

        let initialiser = if self.tokens.matches(&[TokenKind::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.tokens.consume(TokenKind::Semicolon)?;

        Ok(Statement::VariableDeclaration {
            identifier,
            initialiser,
        })
    }

    /// Attempts to parse a function definition. Corresponds to `functionDefinition` in the grammar.
    fn function_definition(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::Fu)?;

        let identifier = self.tokens.consume_identifier()?;

        self.tokens.consume(TokenKind::LeftParenthesis)?;

        let mut parameters = Vec::new();

        if let Ok(parameter) = self.tokens.consume_identifier() {
            parameters.push(parameter);

            while self.tokens.matches(&[TokenKind::Comma]) {
                parameters.push(self.tokens.consume_identifier()?);
            }
        }

        self.tokens.consume(TokenKind::RightParenthesis)?;

        let block = Box::new(self.block()?);

        Ok(Statement::FunctionDefinition {
            identifier,
            parameters,
            block,
        })
    }

    /// Attempts to parse a return statement. Corresponds to `returnStatement` in the grammar.
    fn return_statement(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::Return)?;

        if self.tokens.matches(&[TokenKind::Semicolon]) {
            Ok(Statement::Return(None))
        } else {
            let expression = self.expression()?;
            self.tokens.consume(TokenKind::Semicolon)?;
            Ok(Statement::Return(Some(expression)))
        }
    }

    /// Attempts to parse an if-statement. Corresponds to `ifStatement` in the grammar.
    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::If)?;

        let condition = self.expression()?;

        let execute_if_true = Box::new(self.block()?);

        let execute_if_false = if self.tokens.matches(&[TokenKind::Else]) {
            match self
                .tokens
                .peek()
                .map(|token| (token.kind(), token.location()))
            {
                Some((TokenKind::If, _)) => Some(Box::new(self.if_statement()?)),
                Some((TokenKind::LeftBrace, _)) => Some(Box::new(self.block()?)),
                Some((_, location)) => Err(ParserError::ExpectedToken {
                    expected: vec![TokenKind::If, TokenKind::LeftBrace],
                    location: GeneralLocation::Location(location),
                })?,
                None => Err(ParserError::ExpectedToken {
                    expected: vec![TokenKind::If, TokenKind::LeftBrace],
                    location: GeneralLocation::EndOfFile,
                })?,
            }
        } else {
            None
        };

        Ok(Statement::IfStatement {
            condition,
            execute_if_true,
            execute_if_false,
        })
    }

    /// Attempts to parse a while-loop. Corresponds to `whileLoop` in the grammar.
    fn while_loop(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::While)?;

        let condition = self.expression()?;

        let block = Box::new(self.block()?);

        Ok(Statement::WhileLoop { condition, block })
    }

    /// Attempts to parse a block statement. Corresponds to `block` in the grammar.
    fn block(&mut self) -> Result<Statement, ParserError> {
        self.tokens.consume(TokenKind::LeftBrace)?;

        let mut statements = Vec::new();

        while self
            .tokens
            .peek()
            .is_some_and(|token| token.kind() != TokenKind::RightBrace)
        {
            statements.push(self.statement()?);
        }

        self.tokens.consume(TokenKind::RightBrace)?;

        Ok(Statement::Block(statements))
    }

    /// Attempts to parse an expression statement. Corresponds to `expressionStatement` in the grammar.
    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.expression()?;

        self.tokens.consume(TokenKind::Semicolon)?;

        Ok(Statement::Expression(expression))
    }

    /// Attempts to parse an expression. Corresponds to `expression` in the grammar.
    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.assignment()
    }

    /// Attempts to parse an assignment. Corresponds to `assignment` in the grammar.
    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expression = self.ternary()?;

        if let Some(equals) = self.tokens.only_take(&[TokenKind::Equal]) {
            let value = self.assignment()?;

            match expression {
                Expression::GetField { object, field } => Ok(Expression::SetField {
                    object,
                    field,
                    value: Box::new(value),
                }),
                Expression::Variable { identifier } => Ok(Expression::Assignment {
                    identifier,
                    value: Box::new(value),
                }),
                _ => Err(ParserError::InvalidAssignmentTarget(equals.location())),
            }
        } else {
            Ok(expression)
        }
    }

    /// Attempts to parse a ternary expression. Corresponds to `ternary` in the grammar.
    fn ternary(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.logical()?;

        if self.tokens.matches(&[TokenKind::QuestionMark]) {
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

    /// Attempts to parse a logical expression. Corresponds to `logical` in the grammar.
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

    /// Attempts to parse an equality expression. Corresponds to `equality` in the grammar.
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

    /// Attempts to parse a comparison expression. Corresponds to `comparison` in the grammar.
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

    /// Attempts to parse a bitwise expression. Corresponds to `bitwise` in the grammar.
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

    /// Attempts to parse a term. Corresponds to `term` in the grammar.
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

    /// Attempts to parse a factor. Corresponds to `factor` in the grammar.
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

    /// Attempts to parse a unary expression. Corresponds to `unary` in the grammar.
    fn unary(&mut self) -> Result<Expression, ParserError> {
        if let Some((operator, _)) = self
            .tokens
            .unary_operator(&[UnaryOperator::Minus, UnaryOperator::NOT])
        {
            Ok(Expression::Unary {
                operator: operator,
                operand: Box::new(self.exponent()?),
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
            let _ = self.exponent();

            Err(ParserError::UnsupportedUnaryExpression {
                location: GeneralLocation::Location(location),
                operator: operator,
            })
        } else {
            self.exponent()
        }
    }

    /// Attempts to parse an exponent expression. Corresponds to `exponent` in the grammar.
    fn exponent(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.call()?;

        if self.tokens.matches(&[TokenKind::Exponent]) {
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: BinaryOperator::Exponent,
                right: Box::new(self.exponent()?),
            }
        }

        Ok(expression)
    }

    /// Attempt to parse a call expression. Corresponds to `call` in the grammar.
    fn call(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.primary()?;

        while let Some(token) = self
            .tokens
            .only_take(&[TokenKind::LeftParenthesis, TokenKind::Dot])
        {
            match token.kind() {
                TokenKind::LeftParenthesis => {
                    let mut arguments = Vec::new();

                    if self
                        .tokens
                        .peek()
                        .is_some_and(|token| token.kind() != TokenKind::RightParenthesis)
                    {
                        arguments.push(Box::new(self.expression()?));

                        while self.tokens.matches(&[TokenKind::Comma]) {
                            arguments.push(Box::new(self.expression()?));
                        }
                    }

                    self.tokens.consume(TokenKind::RightParenthesis)?;

                    expression = Expression::Call {
                        function: Box::new(expression),
                        arguments,
                    }
                }
                TokenKind::Dot => {
                    let field = self.tokens.consume_identifier()?;

                    expression = Expression::GetField {
                        object: Box::new(expression),
                        field,
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(expression)
    }

    /// Attempts to parse a primary expression. Corresponds to `primary` in the grammar.
    fn primary(&mut self) -> Result<Expression, ParserError> {
        let expected = [
            TokenKind::LeftParenthesis,
            TokenKind::String,
            TokenKind::Float,
            TokenKind::Integer,
            TokenKind::Boolean,
            TokenKind::Identifier,
            TokenKind::LeftBrace,
        ];

        if let Some(token) = self.tokens.only_take(&expected) {
            Ok(Expression::Literal {
                value: match token.data() {
                    TokenData::LeftParenthesis => {
                        let expression = self.expression()?;

                        self.tokens.consume(TokenKind::RightParenthesis)?;

                        return Ok(Expression::Grouping {
                            contained: Box::new(expression),
                        });
                    }

                    TokenData::String(string) => Value::String(string),

                    TokenData::Float(float) => Value::Float(float),

                    TokenData::Integer(integer) => Value::Integer(integer),

                    TokenData::Boolean(boolean) => Value::Boolean(boolean),

                    TokenData::Identifier(identifier) => {
                        return Ok(Expression::Variable { identifier });
                    }

                    TokenData::LeftBrace => {
                        let mut fields = Vec::new();

                        if self
                            .tokens
                            .peek()
                            .is_some_and(|token| token.kind() != TokenKind::RightBrace)
                        {
                            let identifier = self.tokens.consume_identifier()?;
                            self.tokens.consume(TokenKind::Colon)?;
                            let expression = self.expression()?;
                            fields.push((identifier, expression));

                            while self.tokens.matches(&[TokenKind::Comma]) {
                                let identifier = self.tokens.consume_identifier()?;
                                self.tokens.consume(TokenKind::Colon)?;
                                let expression = self.expression()?;
                                fields.push((identifier, expression));
                            }
                        }

                        self.tokens.consume(TokenKind::RightBrace)?;

                        return Ok(Expression::Object(fields.into_iter().collect()));
                    }

                    _ => unreachable!(),
                },
            })
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

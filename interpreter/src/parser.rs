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

pub enum ParserError {
    ExpectedToken {
        expected: Vec<TokenKind>,
        location: GeneralLocation,
    },
    UnsupportedUnaryExpression {
        operator: BinaryOperator,
        location: GeneralLocation,
    },
    InvalidAssignmentTarget {
        location: Location,
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
            Self::InvalidAssignmentTarget { location } => {
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

pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens }
    }

    pub fn parse(mut self) -> Result<Statement, Vec<ParserError>> {
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
            Ok(Statement::Block { statements })
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
        if self.tokens.matches(&[TokenKind::Print]).is_some() {
            self.print_statement()
        } else if self.tokens.matches(&[TokenKind::Let]).is_some() {
            self.variable_declaration()
        } else if self.tokens.matches(&[TokenKind::Fu]).is_some() {
            self.function_definition()
        } else if self.tokens.matches(&[TokenKind::If]).is_some() {
            self.if_statement()
        } else if self.tokens.matches(&[TokenKind::While]).is_some() {
            self.while_loop()
        } else if self.tokens.matches(&[TokenKind::LeftBrace]).is_some() {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let statement = Statement::Print(self.expression()?);
        self.tokens.consume(TokenKind::Semicolon)?;
        Ok(statement)
    }

    fn variable_declaration(&mut self) -> Result<Statement, ParserError> {
        let identifier = self.tokens.consume_identifier()?;

        let initialiser = if self.tokens.matches(&[TokenKind::Equal]).is_some() {
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

    fn function_definition(&mut self) -> Result<Statement, ParserError> {
        let identifier = self.tokens.consume_identifier()?;

        self.tokens.consume(TokenKind::LeftParenthesis)?;

        let mut parameters = Vec::new();

        if let Ok(parameter) = self.tokens.consume_identifier() {
            parameters.push(parameter);

            while self.tokens.matches(&[TokenKind::Comma]).is_some() {
                parameters.push(self.tokens.consume_identifier()?);
            }
        }

        self.tokens.consume(TokenKind::RightParenthesis)?;

        self.tokens.consume(TokenKind::LeftBrace)?;
        let block = self.block()?;

        Ok(Statement::FunctionDefinition {
            identifier,
            parameters,
            block: Box::new(block),
        })
    }

    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        let condition = self.expression()?;

        self.tokens.consume(TokenKind::LeftBrace)?;
        let execute_if_true = Box::new(self.block()?);

        let execute_if_false = if self.tokens.matches(&[TokenKind::Else]).is_some() {
            if self.tokens.matches(&[TokenKind::If]).is_some() {
                Some(Box::new(self.if_statement()?))
            } else {
                self.tokens.consume(TokenKind::LeftBrace)?;
                Some(Box::new(self.block()?))
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

    fn while_loop(&mut self) -> Result<Statement, ParserError> {
        let condition = self.expression()?;

        self.tokens.consume(TokenKind::LeftBrace)?;
        let block = Box::new(self.block()?);

        Ok(Statement::WhileLoop { condition, block })
    }

    fn block(&mut self) -> Result<Statement, ParserError> {
        let mut statements = Vec::new();

        while self
            .tokens
            .peek()
            .is_some_and(|token| token.kind() != TokenKind::RightBrace)
        {
            statements.push(self.statement()?);
        }

        self.tokens.consume(TokenKind::RightBrace)?;

        Ok(Statement::Block { statements })
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let statement = Statement::Expression(self.expression()?);
        self.tokens.consume(TokenKind::Semicolon)?;
        Ok(statement)
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expression = self.ternary()?;

        if let Some(equals) = self.tokens.matches(&[TokenKind::Equal]) {
            let value = self.assignment()?;

            if let Expression::Variable { identifier } = expression {
                Ok(Expression::Assignment {
                    identifier: identifier,
                    value: Box::new(value),
                })
            } else {
                Err(ParserError::InvalidAssignmentTarget {
                    location: equals.location(),
                })
            }
        } else {
            Ok(expression)
        }
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
                operand: Box::new(self.call()?),
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expression, ParserError> {
        let mut function = self.primary()?;

        while self.tokens.matches(&[TokenKind::LeftParenthesis]).is_some() {
            let mut arguments = Vec::new();

            if self
                .tokens
                .peek()
                .is_some_and(|token| token.kind() != TokenKind::RightParenthesis)
            {
                arguments.push(Box::new(self.expression()?));

                while self.tokens.matches(&[TokenKind::Comma]).is_some() {
                    arguments.push(Box::new(self.expression()?));
                }
            }

            self.tokens.consume(TokenKind::RightParenthesis)?;

            function = Expression::Call {
                function: Box::new(function),
                arguments,
            }
        }

        Ok(function)
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let expected = [
            TokenKind::LeftParenthesis,
            TokenKind::String,
            TokenKind::Float,
            TokenKind::Integer,
            TokenKind::Boolean,
            TokenKind::Identifier,
        ];

        if let Some(token) = self.tokens.matches(&expected) {
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

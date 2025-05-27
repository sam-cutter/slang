use crate::{expression::Expression, token::TokenKind, token_stream::TokenStream};

pub enum ParserError {
    ExpectedToken(Vec<TokenKind>),
}

pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens: tokens }
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
            Err(ParserError::ExpectedToken(vec![
                TokenKind::Bang,
                TokenKind::Minus,
            ]))
        }
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }
}

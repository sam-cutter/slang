use std::collections::VecDeque;

use crate::{
    expression::{BinaryOperator, UnaryOperator},
    parser::ParserError,
    source::{GeneralLocation, Location},
    token::{Token, TokenKind},
};

pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn matches(&mut self, targets: &[TokenKind]) -> Option<Token> {
        if let Some(next) = self.peek() {
            for target in targets {
                if &next.kind() == target {
                    return self.advance();
                }
            }
        }

        None
    }

    pub fn binary_operator(
        &mut self,
        targets: &[BinaryOperator],
    ) -> Option<(BinaryOperator, Location)> {
        if let Some(next) = self.peek() {
            let location = next.location();

            if let Some(operator) = next.kind().binary_operator() {
                for target in targets {
                    if target == &operator {
                        self.advance();
                        return Some((operator, location));
                    }
                }
            }
        }

        None
    }

    pub fn unary_operator(
        &mut self,
        targets: &[UnaryOperator],
    ) -> Option<(UnaryOperator, Location)> {
        if let Some(next) = self.peek() {
            let location = next.location();

            if let Some(operator) = next.kind().unary_operator() {
                for target in targets {
                    if target == &operator {
                        self.advance();
                        return Some((operator, location));
                    }
                }
            }
        }

        None
    }

    pub fn consume(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        if let Some(token) = self.matches(&[kind]) {
            Ok(token)
        } else if let Some(token) = self.peek() {
            Err(ParserError::ExpectedToken {
                expected: vec![kind],
                location: GeneralLocation::Location(token.location()),
            })
        } else {
            Err(ParserError::ExpectedToken {
                expected: vec![kind],
                location: GeneralLocation::EndOfFile,
            })
        }
    }
}

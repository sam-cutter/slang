//! All code relating to the stream of tokens given to the parser.

use std::collections::VecDeque;

use crate::{
    expression::{BinaryOperator, UnaryOperator},
    parser::ParserError,
    source::{GeneralLocation, Location},
    token::{Token, TokenData, TokenKind},
};

/// A wrapper around a queue of tokens.
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    /// Creates a new token stream from a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    /// Returns a reference to the next token in the stream.
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }

    /// Consumes the next token and returns it.
    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    /// Consumes and returns the next token only if it matches a target.
    pub fn only_take(&mut self, targets: &[TokenKind]) -> Option<Token> {
        if let Some(next) = self.peek() {
            for target in targets {
                if &next.kind() == target {
                    return self.advance();
                }
            }
        }

        None
    }

    /// Consumes the next token only if it matches a target. The token is not returned.
    pub fn matches(&mut self, targets: &[TokenKind]) -> bool {
        if let Some(next) = self.peek() {
            for target in targets {
                if &next.kind() == target {
                    self.advance();
                    return true;
                }
            }
        }

        false
    }

    /// Consumes the next token only if it is a binary operator and matches a target.
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

    /// Consumes the next token only if it is a unary operator and matches a target.
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

    /// Consumes the next token only if it is an identifier. Will return an error if it is not an identifier.
    pub fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.peek().cloned();

        match token.map(|token| (token.location(), token.data())) {
            Some((_, TokenData::Identifier(identifier))) => {
                self.advance();
                Ok(identifier)
            }
            Some((location, _)) => Err(ParserError::ExpectedToken {
                expected: vec![TokenKind::Identifier],
                location: GeneralLocation::Location(location),
            }),
            None => Err(ParserError::ExpectedToken {
                expected: vec![TokenKind::Identifier],
                location: GeneralLocation::EndOfFile,
            }),
        }
    }

    /// Consumes the next token only if it is of a certain kind. Will return an error if it is not of that kind.
    pub fn consume(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        if let Some(token) = self.only_take(&[kind]) {
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

    /// Returns whether all of the tokens have been consumed.
    pub fn at_end(&self) -> bool {
        self.tokens.is_empty()
    }
}

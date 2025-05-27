use std::collections::VecDeque;

use crate::{
    parser::ParserError,
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

    pub fn consume(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        if let Some(token) = self.matches(&[kind]) {
            Ok(token)
        } else {
            Err(ParserError::ExpectedToken(vec![kind]))
        }
    }
}

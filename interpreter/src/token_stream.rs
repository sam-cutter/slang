use std::collections::VecDeque;

use crate::token::{Token, TokenKind};

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
                if next.has_kind(target) {
                    return self.advance();
                }
            }
        }

        None
    }
}

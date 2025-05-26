use crate::token_stream::TokenStream;

pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens: tokens }
    }
}

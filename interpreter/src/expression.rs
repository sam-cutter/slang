use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

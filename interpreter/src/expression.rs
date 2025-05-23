use crate::token::Token;

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

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

use crate::expression::Expression;

pub enum Statement {
    Print(Expression),
    Expression(Expression),
}

impl Statement {
    pub fn execute(&self) {
        todo!()
    }
}

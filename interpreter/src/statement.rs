use crate::expression::{EvaluationError, Expression};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        initialiser: Expression,
    },
}

impl Statement {
    pub fn execute(self) -> Result<(), EvaluationError> {
        match self {
            Self::Print(expression) => Ok(println!("{}", expression.evaluate()?)),
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => todo!(),
            Self::Expression(expression) => match expression.evaluate() {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
        }
    }
}

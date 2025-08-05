use crate::{
    environment::Environment,
    expression::{EvaluationError, Expression},
};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        initialiser: Expression,
    },
}

impl Statement {
    pub fn execute(self, environment: &mut Environment) -> Result<(), EvaluationError> {
        match self {
            Self::Print(expression) => Ok(println!("{}", expression.evaluate(environment)?)),
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = initialiser.evaluate(environment)?;
                Ok(environment.define(identifier, initialiser))
            }
            Self::Expression(expression) => match expression.evaluate(environment) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
        }
    }
}

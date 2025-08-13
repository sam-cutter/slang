use crate::{
    environment::Environment,
    expression::{EvaluationError, Expression},
};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        initialiser: Option<Expression>,
    },
    Block {
        statements: Vec<Statement>,
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
                let initialiser = match initialiser {
                    Some(initialiser) => Some(initialiser.evaluate(environment)?),
                    None => None,
                };

                Ok(environment.define(identifier, initialiser))
            }
            Self::Expression(expression) => match expression.evaluate(environment) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
            Self::Block { statements } => {
                environment.enter_scope();

                for statement in statements {
                    statement.execute(environment)?;
                }

                environment.exit_scope();

                Ok(())
            }
        }
    }
}

use crate::{
    expression::{EvaluationError, Expression},
    stack::Stack,
};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        initialiser: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
}

impl Statement {
    pub fn execute(self, stack: &mut Stack) -> Result<(), EvaluationError> {
        match self {
            Self::Print(expression) => Ok(println!("{}", expression.evaluate(stack)?)),
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = initialiser.evaluate(stack)?;
                Ok(stack.define(identifier, initialiser))
            }
            Self::Expression(expression) => match expression.evaluate(stack) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
            Self::Block { statements } => {
                for statement in statements {
                    statement.execute(stack)?;
                }

                stack.pop();

                Ok(())
            }
        }
    }
}

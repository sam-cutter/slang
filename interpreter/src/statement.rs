use crate::{
    environment::Environment,
    expression::{EvaluationError, Expression},
    value::Value,
};

pub enum Statement {
    Print(Expression),
    VariableDeclaration {
        identifier: String,
        initialiser: Option<Expression>,
    },
    IfStatement {
        condition: Expression,
        execute_if_true: Box<Statement>,
        execute_if_false: Option<Box<Statement>>,
    },
    Block {
        statements: Vec<Statement>,
    },
    Expression(Expression),
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
            Self::IfStatement {
                condition,
                execute_if_true,
                execute_if_false,
            } => {
                if let Value::Boolean(condition) = condition.evaluate(environment)? {
                    if condition {
                        execute_if_true.execute(environment)
                    } else {
                        match execute_if_false {
                            Some(if_false) => if_false.execute(environment),
                            None => Ok(()),
                        }
                    }
                } else {
                    // TODO: Add correct evaluation error
                    todo!()
                }
            }
            Self::Block { statements } => {
                environment.enter_scope();

                for statement in statements {
                    statement.execute(environment)?;
                }

                environment.exit_scope();

                Ok(())
            }
            Self::Expression(expression) => match expression.evaluate(environment) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
        }
    }
}

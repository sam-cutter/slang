use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    expression::{EvaluationError, Expression},
    value::{Function, Value},
};

pub enum ControlFlow {
    Continue,
    Break(Option<Value>),
}

#[derive(Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: String,
        initialiser: Option<Expression>,
    },
    IfStatement {
        condition: Expression,
        execute_if_true: Box<Statement>,
        execute_if_false: Option<Box<Statement>>,
    },
    FunctionDefinition {
        identifier: String,
        parameters: Vec<String>,
        block: Box<Statement>,
    },
    Return(Option<Expression>),
    WhileLoop {
        condition: Expression,
        block: Box<Statement>,
    },
    Block {
        statements: Vec<Statement>,
    },
    Expression(Expression),
}

impl Statement {
    pub fn execute(
        self,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<ControlFlow, EvaluationError> {
        match self {
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = match initialiser {
                    Some(initialiser) => {
                        Some(initialiser.evaluate_not_nothing(Rc::clone(&environment))?)
                    }
                    None => None,
                };
                environment.borrow_mut().define(identifier, initialiser);
                Ok(ControlFlow::Continue)
            }
            Self::FunctionDefinition {
                identifier,
                parameters,
                block,
            } => {
                environment.borrow_mut().define(
                    identifier,
                    Some(Value::Function(Function::UserDefined { parameters, block })),
                );
                Ok(ControlFlow::Continue)
            }
            Self::IfStatement {
                condition,
                execute_if_true,
                execute_if_false,
            } => {
                let condition = condition.evaluate_not_nothing(Rc::clone(&environment))?;

                if let Value::Boolean(condition) = condition {
                    if condition {
                        execute_if_true.execute(Rc::clone(&environment))
                    } else {
                        match execute_if_false {
                            Some(if_false) => if_false.execute(Rc::clone(&environment)),
                            None => Ok(ControlFlow::Continue),
                        }
                    }
                } else {
                    Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: "if-statement".to_string(),
                    })
                }
            }
            Self::WhileLoop { condition, block } => {
                while match condition
                    .clone()
                    .evaluate_not_nothing(Rc::clone(&environment))?
                {
                    Value::Boolean(condition) => condition,
                    condition => Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: "while-loop".to_string(),
                    })?,
                } {
                    match block.clone().execute(Rc::clone(&environment))? {
                        ControlFlow::Break(value) => return Ok(ControlFlow::Break(value)),
                        ControlFlow::Continue => continue,
                    }
                }

                Ok(ControlFlow::Continue)
            }
            Self::Block { statements } => {
                let block_scope = Rc::new(RefCell::new(Environment::new(Some(environment))));

                let mut non_definitions = Vec::new();

                for statement in statements {
                    match statement {
                        Statement::FunctionDefinition {
                            identifier: _,
                            parameters: _,
                            block: _,
                        } => {
                            statement.execute(Rc::clone(&block_scope))?;
                        }
                        _ => non_definitions.push(statement),
                    }
                }

                for statement in non_definitions {
                    match statement.execute(Rc::clone(&block_scope))? {
                        ControlFlow::Break(value) => return Ok(ControlFlow::Break(value)),
                        ControlFlow::Continue => continue,
                    }
                }

                Ok(ControlFlow::Continue)
            }
            Self::Expression(expression) => match expression.evaluate(environment) {
                Ok(_) => Ok(ControlFlow::Continue),
                Err(error) => Err(error),
            },
            Self::Return(expression) => match expression {
                Some(expression) => Ok(ControlFlow::Break(
                    expression.evaluate(Rc::clone(&environment))?,
                )),
                None => Ok(ControlFlow::Break(None)),
            },
        }
    }
}

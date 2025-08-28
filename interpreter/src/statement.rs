use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    environment::Environment,
    expression::{EvaluationError, Expression},
    value::Value,
};

pub enum ControlFlow {
    If,
    While,
}

impl Display for ControlFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If => "if-statement",
                Self::While => "while-loop",
            }
        )
    }
}

#[derive(Clone)]
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
    FunctionDefinition {
        identifier: String,
        parameters: Vec<String>,
        block: Box<Statement>,
    },
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
    pub fn execute(self, environment: Rc<RefCell<Environment>>) -> Result<(), EvaluationError> {
        match self {
            Self::Print(expression) => Ok(println!("{}", expression.evaluate(environment)?)),
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = match initialiser {
                    Some(initialiser) => Some(initialiser.evaluate(Rc::clone(&environment))?),
                    None => None,
                };

                Ok(environment.borrow_mut().define(identifier, initialiser))
            }
            Self::FunctionDefinition {
                identifier,
                parameters,
                block,
            } => Ok(environment
                .borrow_mut()
                .define(identifier, Some(Value::Function { parameters, block }))),
            Self::IfStatement {
                condition,
                execute_if_true,
                execute_if_false,
            } => {
                let condition = condition.evaluate(Rc::clone(&environment))?;

                if let Value::Boolean(condition) = condition {
                    if condition {
                        execute_if_true.execute(Rc::clone(&environment))
                    } else {
                        match execute_if_false {
                            Some(if_false) => if_false.execute(Rc::clone(&environment)),
                            None => Ok(()),
                        }
                    }
                } else {
                    Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: ControlFlow::If,
                    })
                }
            }
            Self::WhileLoop { condition, block } => Ok({
                while match condition.clone().evaluate(Rc::clone(&environment))? {
                    Value::Boolean(condition) => condition,
                    condition => Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: ControlFlow::While,
                    })?,
                } {
                    block.clone().execute(Rc::clone(&environment))?;
                }
            }),
            Self::Block { statements } => {
                let block_scope = Rc::new(RefCell::new(Environment::new(Some(environment))));

                let mut non_definitions = Vec::new();

                for statement in statements {
                    match statement {
                        Statement::FunctionDefinition {
                            identifier: _,
                            parameters: _,
                            block: _,
                        } => statement.execute(Rc::clone(&block_scope))?,
                        _ => non_definitions.push(statement),
                    }
                }

                for statement in non_definitions {
                    statement.execute(Rc::clone(&block_scope))?;
                }

                Ok(())
            }
            Self::Expression(expression) => match expression.evaluate(environment) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
        }
    }
}

use crate::{
    expression::{EvaluationError, Expression},
    heap::ManagedHeap,
    stack::Stack,
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
        stack: &mut Stack,
        heap: &mut ManagedHeap,
    ) -> Result<ControlFlow, EvaluationError> {
        match self {
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = match initialiser {
                    Some(initialiser) => Some(initialiser.evaluate_not_nothing(stack, heap)?),
                    None => None,
                };

                // TODO: increment count if necessary
                stack.top().borrow_mut().define(identifier, initialiser);
                Ok(ControlFlow::Continue)
            }
            Self::FunctionDefinition {
                identifier,
                parameters,
                block,
            } => {
                stack.top().borrow_mut().define(
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
                let condition = condition.evaluate_not_nothing(stack, heap)?;

                if let Value::Boolean(condition) = condition {
                    if condition {
                        execute_if_true.execute(stack, heap)
                    } else {
                        match execute_if_false {
                            Some(if_false) => if_false.execute(stack, heap),
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
                while match condition.clone().evaluate_not_nothing(stack, heap)? {
                    Value::Boolean(condition) => condition,
                    condition => Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: "while-loop".to_string(),
                    })?,
                } {
                    match block.clone().execute(stack, heap)? {
                        ControlFlow::Break(value) => return Ok(ControlFlow::Break(value)),
                        ControlFlow::Continue => continue,
                    }
                }

                Ok(ControlFlow::Continue)
            }
            Self::Block { statements } => {
                stack.enter_scope();

                let mut non_definitions = Vec::new();

                for statement in statements {
                    match statement {
                        Statement::FunctionDefinition {
                            identifier: _,
                            parameters: _,
                            block: _,
                        } => {
                            statement.execute(stack, heap)?;
                        }
                        _ => non_definitions.push(statement),
                    }
                }

                let mut return_value = ControlFlow::Continue;

                for statement in non_definitions {
                    match statement.execute(stack, heap)? {
                        ControlFlow::Break(value) => {
                            return_value = ControlFlow::Break(value);
                            break;
                        }
                        ControlFlow::Continue => continue,
                    }
                }

                if let ManagedHeap::ReferenceCounted(heap) = heap {
                    for value in stack.top().borrow().values() {
                        if let Value::ObjectReference(pointer) = value {
                            heap.decrement(pointer);
                        }
                    }
                }

                stack.exit_scope();

                if let ManagedHeap::GarbageCollected(heap) = heap {
                    heap.manage(&stack.roots());
                }

                Ok(return_value)
            }
            Self::Expression(expression) => match expression.evaluate(stack, heap) {
                Ok(_) => Ok(ControlFlow::Continue),
                Err(error) => Err(error),
            },
            Self::Return(expression) => match expression {
                Some(expression) => Ok(ControlFlow::Break(expression.evaluate(stack, heap)?)),
                None => Ok(ControlFlow::Break(None)),
            },
        }
    }
}

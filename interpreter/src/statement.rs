//! Statements within the slang programming language.

use crate::{
    expression::{EvaluationError, Expression},
    heap::{ManagedHeap, Pointer},
    stack::Stack,
    stats::Logger,
    value::{Function, Value},
};

/// Used to signal whether a block should be exited early.
pub enum ControlFlow {
    /// Signals that execution of the block should continue.
    Continue,
    /// Signals that execution of the block should terminate, with an optional value returned.
    Break(Option<Value>),
}

/// Represents a statement.
#[derive(Clone, PartialEq)]
pub enum Statement {
    /// A variable declaration.
    VariableDeclaration {
        identifier: String,
        initialiser: Option<Expression>,
    },
    /// An if-statement.
    IfStatement {
        condition: Expression,
        execute_if_true: Box<Statement>,
        execute_if_false: Option<Box<Statement>>,
    },
    /// A function definition.
    FunctionDefinition {
        identifier: String,
        parameters: Vec<String>,
        block: Box<Statement>,
    },
    /// A return statement.
    Return(Option<Expression>),
    WhileLoop {
        condition: Expression,
        block: Box<Statement>,
    },
    /// A block.
    Block(Vec<Statement>),
    /// An expression statement.
    Expression(Expression),
}

impl Statement {
    /// Executes a statement and inserts a log entry.
    pub fn execute(
        self,
        stack: &mut Stack,
        heap: &mut ManagedHeap,
        logger: &mut Logger,
    ) -> Result<ControlFlow, EvaluationError> {
        stack.top().borrow_mut().define(
            String::from("STACK_FRAMES_COUNT"),
            Some(Value::Integer(stack.frames_count() as i32)),
        );

        stack.top().borrow_mut().define(
            String::from("HEAP_OBJECTS_COUNT"),
            Some(Value::Integer(heap.objects_count() as i32)),
        );

        logger.new_entry(heap.objects_count(), stack.frames_count());

        match self {
            Self::VariableDeclaration {
                identifier,
                initialiser,
            } => {
                let initialiser = match initialiser {
                    Some(initialiser) => {
                        Some(initialiser.evaluate_not_nothing(stack, heap, logger)?)
                    }
                    None => None,
                };

                let previous = stack.top().borrow().get(&identifier);

                let initialiser = match initialiser {
                    Some(Value::Object(data)) => Some(Value::ObjectReference(heap.allocate(data))),
                    Some(Value::ObjectReference(ref pointer)) => {
                        if let ManagedHeap::ReferenceCounted(heap) = heap {
                            heap.increment(Pointer::clone(pointer));
                        }

                        initialiser
                    }
                    _ => initialiser,
                };

                if let (Ok(previous), ManagedHeap::ReferenceCounted(heap)) = (previous, heap) {
                    heap.conditionally_decrement(previous);
                }

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
                let condition = condition.evaluate_not_nothing(stack, heap, logger)?;

                if let Value::Boolean(condition) = condition {
                    if condition {
                        execute_if_true.execute(stack, heap, logger)
                    } else {
                        match execute_if_false {
                            Some(if_false) => if_false.execute(stack, heap, logger),
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
                    .evaluate_not_nothing(stack, heap, logger)?
                {
                    Value::Boolean(condition) => condition,
                    condition => Err(EvaluationError::NonBooleanControlFlowCondition {
                        condition: condition.slang_type(),
                        control_flow: "while-loop".to_string(),
                    })?,
                } {
                    match block.clone().execute(stack, heap, logger)? {
                        ControlFlow::Break(value) => return Ok(ControlFlow::Break(value)),
                        ControlFlow::Continue => continue,
                    }
                }

                Ok(ControlFlow::Continue)
            }
            Self::Block(statements) => {
                stack.enter_scope();

                let mut non_definitions = Vec::new();

                for statement in statements {
                    match statement {
                        Statement::FunctionDefinition { .. } => {
                            statement.execute(stack, heap, logger)?;
                        }
                        _ => non_definitions.push(statement),
                    }
                }

                let mut return_value = ControlFlow::Continue;

                for statement in non_definitions {
                    match statement.execute(stack, heap, logger)? {
                        ControlFlow::Break(value) => {
                            return_value = ControlFlow::Break(value);
                            break;
                        }
                        ControlFlow::Continue => continue,
                    }
                }

                if let ManagedHeap::ReferenceCounted(heap) = heap {
                    if let ControlFlow::Break(Some(Value::ObjectReference(value))) = &return_value {
                        heap.increment(Pointer::clone(value));
                    }

                    for value in stack.top().borrow().values() {
                        heap.conditionally_decrement(value);
                    }
                }

                stack.exit_scope(heap);

                if let ManagedHeap::GarbageCollected(heap) = heap {
                    let mut roots = stack.roots();

                    if let ControlFlow::Break(Some(Value::ObjectReference(pointer))) = &return_value
                    {
                        roots.push(Pointer::clone(pointer));
                    }

                    heap.manage(&roots);
                }

                Ok(return_value)
            }
            Self::Expression(expression) => match expression.evaluate(stack, heap, logger) {
                Ok(_) => Ok(ControlFlow::Continue),
                Err(error) => Err(error),
            },
            Self::Return(expression) => match expression {
                Some(expression) => Ok(ControlFlow::Break(
                    expression.evaluate(stack, heap, logger)?,
                )),
                None => Ok(ControlFlow::Break(None)),
            },
        }
    }
}

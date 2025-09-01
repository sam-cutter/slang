//! Expressions within the slang programming language.

use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    environment::{Environment, EnvironmentError},
    statement::ControlFlow,
    value::{Function, NativeFunction, Type, Value},
};

/// All errors which can occur while evaluating an expression.
pub enum EvaluationError {
    /// When the value of the condition for a ternary expression does not have the type of Boolean.
    NonBooleanTernaryCondition {
        condition: Type,
    },
    /// When the types of the operands for a binary operation are not valid.
    InvalidBinaryTypes {
        left: Type,
        operator: BinaryOperator,
        right: Option<Type>,
    },
    /// When the type of the operand for a unary operation is not valid.
    InvalidUnaryType {
        operator: UnaryOperator,
        operand: Type,
    },
    /// When a division by zero occurs.
    DivisionByZero,
    /// When there is an attempt to get the value of a variable which has not been defined.
    UndefinedIdentifier {
        identifier: String,
    },
    /// When there is an attempt to get the value of a variable which has not been initialised.
    UninitialisedTarget {
        identifier: String,
    },
    /// When the value of the condition for a control flow statement does not have the type of Boolean.
    NonBooleanControlFlowCondition {
        condition: Type,
        control_flow: String,
    },
    AttemptedCallOfNonFunction {
        attempt: Type,
    },
    IncorrectArgumentCount {
        expected: usize,
        passed: usize,
    },
    AttemptToUseNothing,
    AttemptToAccessNonObject {
        attempt: Type,
    },
    UndefinedField(String),
}

impl From<EnvironmentError> for EvaluationError {
    fn from(value: EnvironmentError) -> Self {
        match value {
            EnvironmentError::UndefinedAssignmentTarget { identifier } => {
                Self::UndefinedIdentifier { identifier }
            }
            EnvironmentError::UndefinedTarget { identifier } => {
                Self::UndefinedIdentifier { identifier }
            }
            EnvironmentError::UninitialisedTarget { identifier } => {
                Self::UninitialisedTarget { identifier }
            }
        }
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[evaluation error] ")?;

        match self {
            Self::NonBooleanTernaryCondition { condition } => write!(
                f,
                "Expected Boolean operand for ternary condition, found {}.",
                condition
            ),
            Self::InvalidBinaryTypes {
                left,
                operator,
                right,
            } => write!(
                f,
                "The `{}` operator is not defined for {}{}.",
                operator.raw(),
                left,
                match right {
                    Some(right) => format!(" and {}", right),
                    None => "".to_string(),
                }
            ),
            Self::InvalidUnaryType { operator, operand } => write!(
                f,
                "The unary `{}` operator is not defined for {}.",
                operator.raw(),
                operand
            ),
            Self::DivisionByZero => {
                write!(f, "Division by zero.")
            }
            Self::UndefinedIdentifier { identifier } => {
                write!(f, "The identifier `{}` is not defined.", identifier)
            }
            Self::UninitialisedTarget { identifier } => {
                write!(f, "The target `{}` has not been initialised.", identifier)
            }
            Self::NonBooleanControlFlowCondition {
                condition,
                control_flow,
            } => {
                write!(
                    f,
                    "Expected Boolean {} condition, found {}.",
                    control_flow, condition
                )
            }
            Self::AttemptedCallOfNonFunction { attempt } => {
                write!(
                    f,
                    "Attempted to 'call' a value of type {} like a function.",
                    attempt
                )
            }
            Self::IncorrectArgumentCount { expected, passed } => {
                write!(
                    f,
                    "Expected {} arguments, but received {}.",
                    expected, passed
                )
            }
            Self::AttemptToUseNothing => write!(
                f,
                "Attempted to use the return value from a function, however the function returned nothing."
            ),
            Self::AttemptToAccessNonObject { attempt } => write!(
                f,
                "Attempted to access a field of a value of type {}, like an object.",
                attempt
            ),
            Self::UndefinedField(identifier) => {
                write!(
                    f,
                    "Attempted to access a non-existent field `{}` on an object.",
                    identifier
                )
            }
        }
    }
}

impl Debug for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for EvaluationError {}

/// Represents all possible expressions within the slang programming language.
#[derive(Clone)]
pub enum Expression {
    /// Ternary expressions, in the form `condition ? if_true : if_false`.
    Ternary {
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Binary expressions, in the form `left operator right`.
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    /// Unary expressions, in the form `operator operand`.
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    /// A function call.
    Call {
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    },
    /// An assignment expression, which yields the assigned value.
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    /// An expression surrounded by parenthesis.
    Grouping {
        contained: Box<Expression>,
    },
    /// A literal value.
    Literal {
        value: Value,
    },
    /// A reference to a variable.
    Variable {
        identifier: String,
    },
    FieldAccess {
        object: Box<Expression>,
        field: String,
    },
    Object(HashMap<String, Expression>),
}

impl Expression {
    /// Evaluates an expression, returning an error if it is nothing.
    pub fn evaluate_not_nothing(
        self,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Value, EvaluationError> {
        self.evaluate(environment).map(|value| match value {
            Some(value) => Ok(value),
            None => Err(EvaluationError::AttemptToUseNothing),
        })?
    }

    /// Evaluates the expression.
    pub fn evaluate(
        self,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Option<Value>, EvaluationError> {
        match self {
            Self::Ternary {
                condition,
                left,
                right,
            } => Expression::evaluate_ternary(environment, condition, left, right),

            Self::Binary {
                left,
                operator,
                right,
            } => Expression::evaluate_binary(environment, left, operator, right),

            Self::Unary { operator, operand } => {
                Expression::evaluate_unary(environment, operator, operand)
            }

            Self::Call {
                function,
                arguments,
            } => Expression::evaluate_call(environment, function, arguments),

            Self::Assignment { identifier, value } => {
                let value = value.evaluate(Rc::clone(&environment))?;

                environment.borrow_mut().assign(identifier, value.clone())?;

                Ok(value)
            }

            Self::Grouping { contained } => contained.evaluate(environment),

            Self::Literal { value } => Ok(Some(value)),

            Self::Variable { identifier } => Ok(Some(environment.borrow().get(&identifier)?)),

            Self::FieldAccess { object, field } => {
                match object.evaluate_not_nothing(Rc::clone(&environment))? {
                    Value::Object(fields) => {
                        if let Some(value) = fields.get(&field).cloned() {
                            Ok(Some(value))
                        } else {
                            Err(EvaluationError::UndefinedField(field))
                        }
                    }
                    attempt => Err(EvaluationError::AttemptToAccessNonObject {
                        attempt: attempt.slang_type(),
                    }),
                }
            }

            Self::Object(unevaluated_fields) => {
                let mut fields = HashMap::new();

                for (identifier, expression) in unevaluated_fields.into_iter() {
                    fields.insert(
                        identifier,
                        expression.evaluate_not_nothing(Rc::clone(&environment))?,
                    );
                }

                Ok(Some(Value::Object(fields)))
            }
        }
    }

    /// Evaluates a ternary expression.
    fn evaluate_ternary(
        environment: Rc<RefCell<Environment>>,
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Result<Option<Value>, EvaluationError> {
        let condition = condition.evaluate_not_nothing(Rc::clone(&environment))?;

        if let Value::Boolean(condition) = condition {
            if condition {
                return left.evaluate(Rc::clone(&environment));
            } else {
                return right.evaluate(Rc::clone(&environment));
            }
        } else {
            return Err(EvaluationError::NonBooleanTernaryCondition {
                condition: condition.slang_type(),
            });
        }
    }

    /// Evaluates a binary expression.
    fn evaluate_binary(
        environment: Rc<RefCell<Environment>>,
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    ) -> Result<Option<Value>, EvaluationError> {
        Ok(Some(match operator {
            BinaryOperator::Add => match Self::binary_operands(left, right, environment)? {
                (Value::String(left), Value::String(right)) => {
                    let mut new = left;
                    new.push_str(&right);
                    Value::String(new)
                }
                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
                (Value::Float(left), Value::Float(right)) => Value::Float(left + right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::Subtract => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - right),
                (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::Multiply => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
                (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::Divide => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => {
                    if right == 0 {
                        return Err(EvaluationError::DivisionByZero);
                    }

                    Value::Integer(left / right)
                }
                (Value::Float(left), Value::Float(right)) => {
                    if right == 0.0 {
                        return Err(EvaluationError::DivisionByZero);
                    }

                    Value::Float(left / right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::EqualTo => match Self::binary_operands(left, right, environment)? {
                (Value::String(left), Value::String(right)) => Value::Boolean(left == right),
                (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left == right),
                (Value::Float(left), Value::Float(right)) => Value::Boolean(left == right),
                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::NotEqualTo => match Self::binary_operands(left, right, environment)? {
                (Value::String(left), Value::String(right)) => Value::Boolean(left != right),
                (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left != right),
                (Value::Float(left), Value::Float(right)) => Value::Boolean(left != right),

                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::GreaterThan => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left > right),
                (Value::Float(left), Value::Float(right)) => Value::Boolean(left > right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::GreaterThanOrEqualTo => {
                match Self::binary_operands(left, right, environment)? {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left >= right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left >= right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::LessThan => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left < right),
                (Value::Float(left), Value::Float(right)) => Value::Boolean(left < right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::LessThanOrEqualTo => {
                match Self::binary_operands(left, right, environment)? {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left <= right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::AND => match left.evaluate_not_nothing(Rc::clone(&environment))? {
                Value::Boolean(left) => {
                    if left {
                        match right.evaluate_not_nothing(Rc::clone(&environment))? {
                            Value::Boolean(right) => Value::Boolean(left && right),
                            right => Err(EvaluationError::InvalidBinaryTypes {
                                left: Type::Boolean,
                                operator,
                                right: Some(right.slang_type()),
                            })?,
                        }
                    } else {
                        Value::Boolean(false)
                    }
                }
                left => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: None,
                })?,
            },

            BinaryOperator::OR => match left.evaluate_not_nothing(Rc::clone(&environment))? {
                Value::Boolean(left) => {
                    if left {
                        Value::Boolean(true)
                    } else {
                        match right.evaluate_not_nothing(Rc::clone(&environment))? {
                            Value::Boolean(right) => Value::Boolean(left || right),
                            right => Err(EvaluationError::InvalidBinaryTypes {
                                left: Type::Boolean,
                                operator,
                                right: Some(right.slang_type()),
                            })?,
                        }
                    }
                }
                left => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: None,
                })?,
            },

            BinaryOperator::BitwiseAND => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left & right),
                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left & right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },

            BinaryOperator::BitwiseOR => match Self::binary_operands(left, right, environment)? {
                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left | right),
                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left | right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator,
                    right: Some(right.slang_type()),
                })?,
            },
        }))
    }

    /// Evaluates a unary expression.
    fn evaluate_unary(
        environment: Rc<RefCell<Environment>>,
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Option<Value>, EvaluationError> {
        let operand = operand.evaluate_not_nothing(Rc::clone(&environment))?;

        Ok(Some(match operator {
            UnaryOperator::Minus => match operand {
                Value::Integer(operand) => Value::Integer(-operand),
                Value::Float(operand) => Value::Float(-operand),
                _ => Err(EvaluationError::InvalidUnaryType {
                    operator,
                    operand: operand.slang_type(),
                })?,
            },
            UnaryOperator::NOT => match operand {
                Value::Integer(operand) => Value::Integer(!operand),
                Value::Boolean(operand) => Value::Boolean(!operand),
                _ => Err(EvaluationError::InvalidUnaryType {
                    operator,
                    operand: operand.slang_type(),
                })?,
            },
        }))
    }

    /// Evaluates a function call.
    fn evaluate_call(
        environment: Rc<RefCell<Environment>>,
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    ) -> Result<Option<Value>, EvaluationError> {
        match function.evaluate_not_nothing(Rc::clone(&environment))? {
            Value::Function(Function::UserDefined { parameters, block }) => {
                let mut call_scope =
                    Environment::new(Some(environment.borrow().global(Rc::clone(&environment))));

                if parameters.len() != arguments.len() {
                    return Err(EvaluationError::IncorrectArgumentCount {
                        expected: parameters.len(),
                        passed: arguments.len(),
                    });
                }

                for (parameter, argument) in parameters.into_iter().zip(arguments) {
                    let argument = argument.evaluate(Rc::clone(&environment))?;

                    call_scope.define(parameter, argument);
                }

                block
                    .execute(Rc::new(RefCell::new(call_scope)))
                    .map(|control| match control {
                        ControlFlow::Break(value) => value,
                        ControlFlow::Continue => None,
                    })
            }
            Value::Function(Function::Native(function)) => match function {
                NativeFunction::Print => match &arguments[..] {
                    [] => {
                        println!();
                        Ok(None)
                    }
                    [expression] => {
                        // TODO: figure out how to do this without cloning
                        println!(
                            "{}",
                            expression
                                .clone()
                                .evaluate_not_nothing(Rc::clone(&environment))?
                        );
                        Ok(None)
                    }
                    _ => Err(EvaluationError::IncorrectArgumentCount {
                        expected: 1,
                        passed: arguments.len(),
                    }),
                },
                NativeFunction::Format => {
                    let mut buffer = String::new();

                    for argument in arguments {
                        buffer.push_str(&format!(
                            "{}",
                            argument.evaluate_not_nothing(Rc::clone(&environment))?
                        ));
                    }

                    Ok(Some(Value::String(buffer)))
                }
            },
            other => Err(EvaluationError::AttemptedCallOfNonFunction {
                attempt: other.slang_type(),
            }),
        }
    }

    /// Evaluates a set of binary operands, ensuring that they are not nothing.
    fn binary_operands(
        left: Box<Expression>,
        right: Box<Expression>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(Value, Value), EvaluationError> {
        Ok((
            left.evaluate_not_nothing(Rc::clone(&environment))?,
            right.evaluate_not_nothing(Rc::clone(&environment))?,
        ))
    }
}

/// All valid binary operators.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOperator {
    // Arithmetic operators
    Add,
    Subtract,
    Multiply,
    Divide,

    // Logical operators
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    AND,
    OR,

    // Bitwise operators
    BitwiseAND,
    BitwiseOR,
}

impl BinaryOperator {
    /// How the binary operator will appear in source code.
    pub fn raw(&self) -> String {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",

            Self::EqualTo => "==",
            Self::NotEqualTo => "!=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqualTo => ">=",
            Self::LessThan => "<",
            Self::LessThanOrEqualTo => "<=",
            Self::AND => "&&",
            Self::OR => "||",

            Self::BitwiseAND => "&",
            Self::BitwiseOR => "|",
        }
        .to_string()
    }
}

/// All valid unary operators.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    NOT,
}

impl UnaryOperator {
    /// How the unary operator will appear in source code.
    pub fn raw(&self) -> String {
        match self {
            Self::Minus => "-",
            Self::NOT => "!",
        }
        .to_string()
    }
}

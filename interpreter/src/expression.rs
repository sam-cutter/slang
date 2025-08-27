//! Expressions within the slang programming language.

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    environment::{Environment, EnvironmentError},
    statement::ControlFlow,
    value::{Type, Value},
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
    UninitialisedVariable {
        identifier: String,
    },
    /// When the value of the condition for a control flow statement does not have the type of Boolean.
    NonBooleanControlFlowCondition {
        condition: Type,
        control_flow: ControlFlow,
    },
    AttemptedCallOfNonFunction {
        attempt: Type,
    },
    IncorrectArgumentCount {
        expected: usize,
        passed: usize,
    },
}

impl From<EnvironmentError> for EvaluationError {
    fn from(value: EnvironmentError) -> Self {
        match value {
            EnvironmentError::UndefinedAssignmentTarget { identifier } => {
                Self::UndefinedIdentifier { identifier }
            }
            EnvironmentError::UndefinedVariable { identifier } => {
                Self::UndefinedIdentifier { identifier }
            }
            EnvironmentError::UninitialisedVariable { identifier } => {
                Self::UninitialisedVariable { identifier }
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
            Self::UninitialisedVariable { identifier } => {
                write!(f, "The variable `{}` has not been initialised.", identifier)
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
    Grouping { contained: Box<Expression> },
    /// A literal value.
    Literal { value: Value },
    /// A reference to a variable.
    Variable { identifier: String },
}

impl Expression {
    /// Evaluates the expression.
    pub fn evaluate(self, environment: &mut Environment) -> Result<Value, EvaluationError> {
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
                let value = value.evaluate(environment)?;

                environment.assign(identifier, value.clone())?;

                Ok(value)
            }

            Self::Grouping { contained } => contained.evaluate(environment),

            Self::Literal { value } => Ok(value),

            Self::Variable { identifier } => Ok(environment.get(&identifier)?),
        }
    }

    /// Evaluates a ternary expression.
    fn evaluate_ternary(
        environment: &mut Environment,
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Result<Value, EvaluationError> {
        let condition = condition.evaluate(environment)?;

        if let Value::Boolean(condition) = condition {
            if condition {
                return left.evaluate(environment);
            } else {
                return right.evaluate(environment);
            }
        } else {
            return Err(EvaluationError::NonBooleanTernaryCondition {
                condition: condition.slang_type(),
            });
        }
    }

    /// Evaluates a binary expression.
    fn evaluate_binary(
        environment: &mut Environment,
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    ) -> Result<Value, EvaluationError> {
        Ok(match operator {
            BinaryOperator::Add => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
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
                }
            }

            BinaryOperator::Subtract => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - right),
                    (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::Multiply => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
                    (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::Divide => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
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
                }
            }

            BinaryOperator::EqualTo => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::String(left), Value::String(right)) => Value::Boolean(left == right),
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left == right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left == right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::NotEqualTo => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::String(left), Value::String(right)) => Value::Boolean(left != right),
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left != right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left != right),

                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::GreaterThan => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left > right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left > right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::GreaterThanOrEqualTo => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left >= right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left >= right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::LessThan => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left < right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left < right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::LessThanOrEqualTo => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Boolean(left <= right),
                    (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::AND => match left.evaluate(environment)? {
                Value::Boolean(left) => {
                    if left {
                        match right.evaluate(environment)? {
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

            BinaryOperator::OR => match left.evaluate(environment)? {
                Value::Boolean(left) => {
                    if left {
                        Value::Boolean(true)
                    } else {
                        match right.evaluate(environment)? {
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

            BinaryOperator::BitwiseAND => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Integer(left & right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left & right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }

            BinaryOperator::BitwiseOR => {
                match (left.evaluate(environment)?, right.evaluate(environment)?) {
                    (Value::Integer(left), Value::Integer(right)) => Value::Integer(left | right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left | right),
                    (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                        left: left.slang_type(),
                        operator,
                        right: Some(right.slang_type()),
                    })?,
                }
            }
        })
    }

    /// Evaluates a unary expression.
    fn evaluate_unary(
        environment: &mut Environment,
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Value, EvaluationError> {
        let operand = operand.evaluate(environment)?;

        Ok(match operator {
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
        })
    }

    /// Evaluates a function call.
    fn evaluate_call(
        environment: &mut Environment,
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    ) -> Result<Value, EvaluationError> {
        match function.evaluate(environment)? {
            Value::Function { parameters, block } => {
                let mut call_environment = Environment::new();

                if parameters.len() != arguments.len() {
                    return Err(EvaluationError::IncorrectArgumentCount {
                        expected: parameters.len(),
                        passed: arguments.len(),
                    });
                }

                for (parameter, argument) in parameters.into_iter().zip(arguments) {
                    let argument = argument.evaluate(environment)?;

                    call_environment.define(parameter, Some(argument));
                }

                block.execute(&mut call_environment)?;

                // TODO: return any value which was returned from the function
                Ok(Value::Integer(0))
            }
            other => Err(EvaluationError::AttemptedCallOfNonFunction {
                attempt: other.slang_type(),
            }),
        }
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

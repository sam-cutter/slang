use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    environment::{Environment, EnvironmentError},
    value::{Type, Value},
};

pub enum EvaluationError {
    NonBooleanTernaryCondition {
        condition: Type,
    },
    InvalidBinaryTypes {
        left: Type,
        operator: BinaryOperator,
        right: Option<Type>,
    },
    InvalidUnaryType {
        operator: UnaryOperator,
        operand: Type,
    },
    DivisionByZero,
    UndefinedIdentifier {
        identifier: String,
    },
    UninitialisedVariable {
        identifier: String,
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
        match self {
            Self::NonBooleanTernaryCondition { condition } => write!(
                f,
                "[evaluation error] Expected Boolean operand for ternary condition, found {}.",
                condition
            ),
            Self::InvalidBinaryTypes {
                left,
                operator,
                right,
            } => write!(
                f,
                "[evaluation error] The `{}` operator is not defined for {}{}.",
                operator.raw(),
                left,
                match right {
                    Some(right) => format!(" and {}", right),
                    None => "".to_string(),
                }
            ),
            Self::InvalidUnaryType { operator, operand } => write!(
                f,
                "[evaluation error] The unary `{}` operator is not defined for {}.",
                operator.raw(),
                operand
            ),
            Self::DivisionByZero => {
                write!(f, "[evaluation error] Division by zero.")
            }
            Self::UndefinedIdentifier { identifier } => {
                write!(
                    f,
                    "[evaluation error] The identifier `{}` is not defined.",
                    identifier
                )
            }
            Self::UninitialisedVariable { identifier } => {
                write!(
                    f,
                    "[evaluation error] The variable `{}` has not been initialised.",
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

#[derive(Debug)]
pub enum Expression {
    Ternary {
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Value),
    Variable(String),
}

impl Expression {
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

            Self::Assignment { identifier, value } => {
                let value = value.evaluate(environment)?;

                environment.assign(identifier, value.clone())?;

                Ok(value)
            }

            Self::Grouping(expression) => expression.evaluate(environment),

            Self::Literal(literal) => Ok(literal),

            Self::Variable(identifier) => Ok(environment.get(&identifier)?),
        }
    }

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
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Minus,
    NOT,
}

impl UnaryOperator {
    pub fn raw(&self) -> String {
        match self {
            Self::Minus => "-",
            Self::NOT => "!",
        }
        .to_string()
    }
}

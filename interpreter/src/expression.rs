use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::environment::Environment;

pub enum EvaluationError {
    NonBooleanTernaryCondition {
        condition: SlangType,
    },
    InvalidBinaryTypes {
        left: SlangType,
        operator: BinaryOperator,
        right: SlangType,
    },
    InvalidUnaryType {
        operator: UnaryOperator,
        operand: SlangType,
    },
    DivisionByZero,
    UndefinedVariable {
        name: String,
    },
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
                "[evaluation error] The `{}` operator is not defined for {} and {}.",
                operator.raw(),
                left,
                right
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
            Self::UndefinedVariable { name } => {
                write!(
                    f,
                    "[evaluation error] The variable `{}` is not defined.",
                    name
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
    Grouping(Box<Expression>),
    Literal(Literal),
    Variable(String),
}

impl Expression {
    pub fn evaluate(self, environment: &mut Environment) -> Result<Literal, EvaluationError> {
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

            Self::Grouping(expression) => expression.evaluate(environment),

            Self::Literal(literal) => Ok(literal),

            Self::Variable(identifier) => match environment.get(&identifier) {
                Some(literal) => Ok(literal),
                None => Err(EvaluationError::UndefinedVariable { name: identifier }),
            },
        }
    }

    fn evaluate_ternary(
        environment: &mut Environment,
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let condition = condition.evaluate(environment)?;

        if let Literal::Boolean(condition) = condition {
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
    ) -> Result<Literal, EvaluationError> {
        let operands = (left.evaluate(environment)?, right.evaluate(environment)?);

        Ok(match operator {
            BinaryOperator::Add => match operands {
                (Literal::String(left), Literal::String(right)) => {
                    let mut new = left;
                    new.push_str(&right);
                    Literal::String(new)
                }
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left + right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::Subtract => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left - right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left - right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::Multiply => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left * right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left * right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::Divide => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    if right == 0 {
                        return Err(EvaluationError::DivisionByZero);
                    }

                    Literal::Integer(left / right)
                }
                (Literal::Float(left), Literal::Float(right)) => {
                    if right == 0.0 {
                        return Err(EvaluationError::DivisionByZero);
                    }

                    Literal::Float(left / right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::EqualTo => match operands {
                (Literal::String(left), Literal::String(right)) => Literal::Boolean(left == right),
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Boolean(left == right)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left == right),
                (Literal::Boolean(left), Literal::Boolean(right)) => {
                    Literal::Boolean(left == right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::NotEqualTo => match operands {
                (Literal::String(left), Literal::String(right)) => Literal::Boolean(left != right),
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Boolean(left != right)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left != right),

                (Literal::Boolean(left), Literal::Boolean(right)) => {
                    Literal::Boolean(left != right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::GreaterThan => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Boolean(left > right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left > right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::GreaterThanOrEqualTo => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Boolean(left >= right)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left >= right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),

                    operator: operator,

                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::LessThan => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Boolean(left < right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left < right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),

                    operator: operator,

                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::LessThanOrEqualTo => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Boolean(left <= right)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left <= right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::AND => match operands {
                (Literal::Boolean(left), Literal::Boolean(right)) => {
                    Literal::Boolean(left && right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::OR => match operands {
                (Literal::Boolean(left), Literal::Boolean(right)) => {
                    Literal::Boolean(left || right)
                }
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::BitwiseAND => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left & right),
                (Literal::Boolean(left), Literal::Boolean(right)) => Literal::Boolean(left & right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },

            BinaryOperator::BitwiseOR => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left | right),
                (Literal::Boolean(left), Literal::Boolean(right)) => Literal::Boolean(left | right),
                (left, right) => Err(EvaluationError::InvalidBinaryTypes {
                    left: left.slang_type(),
                    operator: operator,
                    right: right.slang_type(),
                })?,
            },
        })
    }

    fn evaluate_unary(
        environment: &mut Environment,
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let operand = operand.evaluate(environment)?;

        Ok(match operator {
            UnaryOperator::Minus => match operand {
                Literal::Integer(operand) => Literal::Integer(-operand),
                Literal::Float(operand) => Literal::Float(-operand),
                _ => Err(EvaluationError::InvalidUnaryType {
                    operator,
                    operand: operand.slang_type(),
                })?,
            },
            UnaryOperator::NOT => match operand {
                Literal::Integer(operand) => Literal::Integer(!operand),
                Literal::Boolean(operand) => Literal::Boolean(!operand),
                _ => Err(EvaluationError::InvalidUnaryType {
                    operator,
                    operand: operand.slang_type(),
                })?,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    Integer(i32),
    Boolean(bool),
    Null,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Null => write!(f, "null"),
        }
    }
}

impl Literal {
    pub fn slang_type(&self) -> SlangType {
        match self {
            Self::String(_) => SlangType::String,
            Self::Float(_) => SlangType::Float,
            Self::Integer(_) => SlangType::Integer,
            Self::Boolean(_) => SlangType::Boolean,
            Self::Null => SlangType::Null,
        }
    }
}

#[derive(Debug)]
pub enum SlangType {
    String,
    Float,
    Integer,
    Boolean,
    Null,
}

impl Display for SlangType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Null => write!(f, "null"),
        }
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

use std::fmt::Display;

pub enum EvaluationError {
    InvalidBinaryTypes {
        left: SlangType,
        operator: BinaryOperator,
        right: SlangType,
    },
    InvalidUnaryType {
        operator: UnaryOperator,
        operand: SlangType,
    },
    NonBooleanTernaryCondition {
        condition: SlangType,
    },
    DivisionByZero,
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBinaryTypes {
                left,
                operator,
                right,
            } => write!(
                f,
                "The `{}` operator is not defined for {} and {}.",
                operator.raw(),
                left,
                right
            ),
            Self::InvalidUnaryType { operator, operand } => write!(
                f,
                "The {} operator is not defined for {}.",
                operator.raw(),
                operand
            ),
            Self::NonBooleanTernaryCondition { condition } => write!(
                f,
                "Expected Boolean operand for ternary condition, found {}.",
                condition
            ),
            Self::DivisionByZero => {
                write!(f, "Division by zero.")
            }
        }
    }
}

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
}

impl Expression {
    pub fn evaluate(self) -> Result<Literal, EvaluationError> {
        match self {
            Self::Ternary {
                condition,
                left,
                right,
            } => Expression::evaluate_ternary(condition, left, right),

            Self::Binary {
                left,
                operator,
                right,
            } => Expression::evaluate_binary(left, operator, right),

            Self::Unary { operator, operand } => Expression::evaluate_unary(operator, operand),

            Self::Grouping(expression) => expression.evaluate(),

            Self::Literal(literal) => Ok(literal),
        }
    }

    fn evaluate_ternary(
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let condition = condition.evaluate()?;

        if let Literal::Boolean(condition) = condition {
            if condition {
                return left.evaluate();
            } else {
                return right.evaluate();
            }
        } else {
            return Err(EvaluationError::NonBooleanTernaryCondition {
                condition: condition.slang_type(),
            });
        }
    }

    fn evaluate_binary(
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let operands = (left.evaluate()?, right.evaluate()?);

        Ok(match operator {
            BinaryOperator::Add => match operands {
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
                (Literal::Integer(left), Literal::Integer(right)) => {
                    Literal::Boolean(left != right)
                }
                (Literal::Float(left), Literal::Float(right)) => Literal::Boolean(left == right),
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
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let operand = operand.evaluate()?;

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

#[derive(Debug)]
pub enum Literal {
    Float(f64),
    Integer(i32),
    Boolean(bool),
}

impl Literal {
    pub fn slang_type(&self) -> SlangType {
        match self {
            Self::Float(_) => SlangType::Float,
            Self::Integer(_) => SlangType::Integer,
            Self::Boolean(_) => SlangType::Boolean,
        }
    }
}

#[derive(Debug)]
pub enum SlangType {
    Float,
    Integer,
    Boolean,
}

impl Display for SlangType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
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

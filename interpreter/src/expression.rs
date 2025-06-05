use std::fmt::Display;

pub enum EvaluationError {
    InvalidBinaryTypes {
        left: SlangType,
        operator: BinaryOperator,
        right: SlangType,
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
                "The `{}` operator is not defined for {} and {}",
                operator.raw(),
                left,
                right
            ),
            Self::DivisionByZero => {
                write!(f, "Division by zero")
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
        if let Literal::Boolean(condition) = condition.evaluate()? {
            if condition {
                return left.evaluate();
            } else {
                return right.evaluate();
            }
        }

        todo!()
    }

    fn evaluate_binary(
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let operands = (left.evaluate()?, right.evaluate()?);

        return Ok(match operator {
            BinaryOperator::Add => match operands {
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left + right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
                (Literal::String(left), Literal::String(right)) => {
                    let mut new = left;
                    new.push_str(&right);
                    Literal::String(new)
                }
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
            },
        });
    }

    fn evaluate_unary(
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        let operand = operand.evaluate()?;

        if let Literal::Integer(integer) = operand {
            return Ok(match operator {
                UnaryOperator::Minus => Literal::Integer(-integer),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        } else if let Literal::Float(float) = operand {
            return Ok(match operator {
                UnaryOperator::Minus => Literal::Float(-float),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        } else if let Literal::Boolean(boolean) = operand {
            return Ok(match operator {
                UnaryOperator::NOT => Literal::Boolean(!boolean),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        }

        todo!()
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Float(f64),
    Integer(i32),
    Boolean(bool),
    Null,
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

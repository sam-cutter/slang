pub enum EvaluationError {}

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

        // TODO: instead match first on operator, then on operands

        if let (Literal::Integer(left), Literal::Integer(right)) = operands {
            return Ok(match operator {
                BinaryOperator::Add => Literal::Integer(left + right),
                BinaryOperator::Subtract => Literal::Integer(left - right),
                BinaryOperator::Multiply => Literal::Integer(left * right),
                // TODO: check for a division by zero
                BinaryOperator::Divide => Literal::Integer(left / right),

                BinaryOperator::EqualTo => Literal::Boolean(left == right),
                BinaryOperator::NotEqualTo => Literal::Boolean(left != right),
                BinaryOperator::GreaterThan => Literal::Boolean(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Literal::Boolean(left >= right),
                BinaryOperator::LessThan => Literal::Boolean(left < right),
                BinaryOperator::LessThanOrEqualTo => Literal::Boolean(left <= right),

                BinaryOperator::BitwiseAND => Literal::Integer(left & right),
                BinaryOperator::BitwiseOR => Literal::Integer(left | right),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        } else if let (Literal::Float(left), Literal::Float(right)) = operands {
            return Ok(match operator {
                BinaryOperator::Add => Literal::Float(left + right),
                BinaryOperator::Subtract => Literal::Float(left - right),
                BinaryOperator::Multiply => Literal::Float(left * right),
                // TODO: check for a division by zero
                BinaryOperator::Divide => Literal::Float(left / right),

                BinaryOperator::EqualTo => Literal::Boolean(left == right),
                BinaryOperator::NotEqualTo => Literal::Boolean(left != right),
                BinaryOperator::GreaterThan => Literal::Boolean(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Literal::Boolean(left >= right),
                BinaryOperator::LessThan => Literal::Boolean(left < right),
                BinaryOperator::LessThanOrEqualTo => Literal::Boolean(left <= right),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        } else if let (Literal::Boolean(left), Literal::Boolean(right)) = operands {
            return Ok(match operator {
                BinaryOperator::EqualTo => Literal::Boolean(left == right),
                BinaryOperator::NotEqualTo => Literal::Boolean(left != right),
                BinaryOperator::AND => Literal::Boolean(left && right),
                BinaryOperator::OR => Literal::Boolean(left || right),

                // TODO: throw an error for operations which can't be performed
                _ => todo!(),
            });
        }

        todo!()
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

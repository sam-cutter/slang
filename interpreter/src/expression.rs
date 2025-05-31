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

        if let (Literal::Number(left), Literal::Number(right)) = operands {
            match operator {
                BinaryOperator::Add => return Ok(Literal::Number(left + right)),
                BinaryOperator::Subtract => return Ok(Literal::Number(left - right)),
                BinaryOperator::Multiply => return Ok(Literal::Number(left * right)),
                // TODO: check for a division by zero
                BinaryOperator::Divide => return Ok(Literal::Number(left / right)),

                BinaryOperator::EqualTo => return Ok(Literal::Boolean(left == right)),
                BinaryOperator::NotEqualTo => return Ok(Literal::Boolean(left != right)),
                BinaryOperator::GreaterThan => return Ok(Literal::Boolean(left > right)),
                BinaryOperator::GreaterThanOrEqualTo => return Ok(Literal::Boolean(left >= right)),
                BinaryOperator::LessThan => return Ok(Literal::Boolean(left < right)),
                BinaryOperator::LessThanOrEqualTo => return Ok(Literal::Boolean(left <= right)),

                // TODO: bitwise operations don't work on floats, need to introduce integer type
                _ => todo!(),
            }
        } else if let (Literal::Boolean(left), Literal::Boolean(right)) = operands {
            match operator {
                BinaryOperator::EqualTo => return Ok(Literal::Boolean(left == right)),
                BinaryOperator::NotEqualTo => return Ok(Literal::Boolean(left != right)),
                BinaryOperator::AND => return Ok(Literal::Boolean(left && right)),
                BinaryOperator::OR => return Ok(Literal::Boolean(left || right)),

                _ => todo!(),
            }
        }

        todo!()
    }

    fn evaluate_unary(
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> Result<Literal, EvaluationError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
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

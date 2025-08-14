use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i32),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
        }
    }
}

impl Value {
    pub fn slang_type(&self) -> Type {
        match self {
            Self::String(_) => Type::String,
            Self::Float(_) => Type::Float,
            Self::Integer(_) => Type::Integer,
            Self::Boolean(_) => Type::Boolean,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    String,
    Float,
    Integer,
    Boolean,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
        }
    }
}

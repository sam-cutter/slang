use std::fmt::Display;

use crate::statement::Statement;

#[derive(Clone)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i32),
    Boolean(bool),
    Function {
        parameters: Vec<String>,
        block: Box<Statement>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Function {
                parameters,
                block: _,
            } => {
                write!(f, "<function with {} named parameters>", parameters.len())
            }
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
            Self::Function {
                parameters: _,
                block: _,
            } => Type::Function,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    String,
    Float,
    Integer,
    Boolean,
    Function,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Function => write!(f, "Function"),
        }
    }
}

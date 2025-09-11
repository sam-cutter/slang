use std::fmt::Display;

use crate::{
    heap::{Object, Pointer},
    statement::Statement,
};

#[derive(Clone)]
pub enum NativeFunction {
    Print,
    Format,
}

#[derive(Clone)]
pub enum Function {
    UserDefined {
        parameters: Vec<String>,
        block: Box<Statement>,
    },
    Native(NativeFunction),
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i32),
    Boolean(bool),
    Function(Function),
    ObjectReference(Pointer),
    Object(Object),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Function(function) => match function {
                Function::Native(_) => write!(f, "<native function>"),
                Function::UserDefined {
                    parameters,
                    block: _,
                } => write!(f, "<function with {} named parameters>", parameters.len()),
            },
            Self::Object(fields) => {
                write!(
                    f,
                    "{{ {} }}",
                    fields
                        .iter()
                        .map(|(identifier, _expression)| format!("{}", identifier))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Self::ObjectReference(_) => {
                write!(f, "<object reference>")
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
            Self::Function(_) => Type::Function,
            Self::Object(_) => Type::Object,
            Self::ObjectReference(_) => Type::Object,
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
    Object,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Function => write!(f, "Function"),
            Self::Object => write!(f, "Object"),
        }
    }
}

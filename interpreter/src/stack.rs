use crate::{
    environment::{Environment, EnvironmentError},
    expression::Literal,
};

pub enum StackError {
    UndefinedAssignmentTarget { identifier: String },
}

impl From<EnvironmentError> for StackError {
    fn from(value: EnvironmentError) -> Self {
        match value {
            EnvironmentError::UndefinedAssignmentTarget { identifier } => {
                StackError::UndefinedAssignmentTarget { identifier }
            }
        }
    }
}

pub struct Stack {
    stack: Vec<Environment>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: vec![Environment::new()],
        }
    }

    pub fn define(&mut self, identifier: String, value: Literal) {
        if let Some(environment) = self.stack.last_mut() {
            environment.define(identifier, value);
        } else {
            let mut environment = Environment::new();

            environment.define(identifier, value);

            self.stack.push(environment);
        }
    }

    pub fn assign(&mut self, identifier: String, value: Literal) -> Result<(), StackError> {
        if self.stack.is_empty() {
            return Err(StackError::UndefinedAssignmentTarget { identifier });
        }

        let mut reverse = self.stack.iter_mut().rev();

        while let Some(environment) = reverse.next() {
            if environment.get(&identifier).is_some() {
                return Ok(environment.assign(identifier, value)?);
            }
        }

        Err(StackError::UndefinedAssignmentTarget { identifier })
    }

    pub fn get(&self, identifier: &str) -> Option<Literal> {
        let mut reverse = self.stack.iter().rev();

        while let Some(environment) = reverse.next() {
            if let Some(value) = environment.get(&identifier) {
                return Some(value);
            }
        }

        None
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

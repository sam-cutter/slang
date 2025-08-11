use std::collections::HashMap;

use crate::expression::Literal;

pub enum EnvironmentError {
    UndefinedAssignmentTarget { identifier: String },
}

pub struct Environment {
    variables: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: String, value: Literal) {
        self.variables.insert(identifier, value);
    }

    pub fn assign(&mut self, identifier: String, value: Literal) -> Result<(), EnvironmentError> {
        if self.variables.contains_key(&identifier) {
            self.variables.insert(identifier, value);
            Ok(())
        } else {
            Err(EnvironmentError::UndefinedAssignmentTarget { identifier })
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Literal> {
        self.variables.get(identifier).cloned()
    }
}

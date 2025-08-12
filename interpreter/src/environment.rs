use std::collections::HashMap;

use crate::expression::Literal;

pub enum EnvironmentError {
    UndefinedAssignmentTarget { identifier: String },
}

pub struct Environment {
    scopes: Vec<HashMap<String, Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn define(&mut self, identifier: String, value: Literal) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(identifier, value);
        } else {
            let mut scope = HashMap::new();

            scope.insert(identifier, value);

            self.scopes.push(scope);
        }
    }

    pub fn assign(&mut self, identifier: String, value: Literal) -> Result<(), EnvironmentError> {
        if self.scopes.is_empty() {
            return Err(EnvironmentError::UndefinedAssignmentTarget { identifier });
        }

        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&identifier) {
                scope.insert(identifier, value);
                return Ok(());
            }
        }

        Err(EnvironmentError::UndefinedAssignmentTarget { identifier })
    }

    pub fn get(&self, identifier: &str) -> Option<Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(identifier).cloned() {
                return Some(value);
            }
        }

        None
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

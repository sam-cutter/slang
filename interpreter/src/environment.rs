use std::collections::HashMap;

use crate::expression::Literal;

pub enum EnvironmentError {
    UndefinedAssignmentTarget { identifier: String },
    UninitialisedVariable { identifier: String },
    UndefinedVariable { identifier: String },
}

pub struct Environment {
    scopes: Vec<HashMap<String, Option<Literal>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn define(&mut self, identifier: String, value: Option<Literal>) {
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
                scope.insert(identifier, Some(value));
                return Ok(());
            }
        }

        Err(EnvironmentError::UndefinedAssignmentTarget { identifier })
    }

    pub fn get(&self, identifier: &str) -> Result<Literal, EnvironmentError> {
        for scope in self.scopes.iter().rev() {
            match scope.get(identifier) {
                Some(Some(value)) => return Ok(value.clone()),
                Some(None) => {
                    return Err(EnvironmentError::UninitialisedVariable {
                        identifier: identifier.to_string(),
                    });
                }
                None => continue,
            }
        }

        Err(EnvironmentError::UndefinedVariable {
            identifier: identifier.to_string(),
        })
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

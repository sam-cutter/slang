//! Environments and scopes for the slang programming language.

use std::collections::HashMap;

use crate::value::Value;

/// All errors which can occur while accessing the environment.
pub enum EnvironmentError {
    /// When there is an attempt to mutate a variable which has not been defined.
    UndefinedAssignmentTarget { identifier: String },
    /// When there is an attempt to get the value of a variable which has not been initialised.
    UninitialisedVariable { identifier: String },
    /// When there is an attempt to get the value of a variable which has not been defined.
    UndefinedVariable { identifier: String },
}

/// An [Environment] represents a set of scopes, stacked on top of one another.
///
/// Note that this is not the same thing as the stack: the stack has a frame for each subroutine call, whereas the environment has a new scope for each block scope (e.g. if-statements, while-loops).
pub struct Environment {
    /// The contained scopes.
    scopes: Vec<HashMap<String, Option<Value>>>,
}

impl Environment {
    /// Creates a new [Environment], with one scope already put in place.
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    /// Defines a new variable and inserts it into the innermost scope.
    pub fn define(&mut self, identifier: String, value: Option<Value>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(identifier, value);
        } else {
            let mut scope = HashMap::new();

            scope.insert(identifier, value);

            self.scopes.push(scope);
        }
    }

    /// Mutates the value of a variable.
    ///
    /// In order to find the variable to mutate, the program starts in the innermost scope and works outwards until the variable is found (or is not found anywhere).
    pub fn assign(&mut self, identifier: String, value: Value) -> Result<(), EnvironmentError> {
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

    /// Gets the value of a variable.
    ///
    /// In order to find the variable, the program starts in the innermost scope and works outwards until the variable is found (or is not found anywhere).
    pub fn get(&self, identifier: &str) -> Result<Value, EnvironmentError> {
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

    /// Pushes a new scope.
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pops the current scope.
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}

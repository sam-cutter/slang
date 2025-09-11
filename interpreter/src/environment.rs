//! Environments and scopes for the slang programming language.

use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

use crate::{
    heap::Pointer,
    value::{Function, NativeFunction, Value},
};

/// All errors which can occur while accessing the environment.
pub enum EnvironmentError {
    /// When there is an attempt to assign a value to a target which has not been defined.
    UndefinedAssignmentTarget { identifier: String },
    /// When there is an attempt to get the value of a target which has not been initialised.
    UninitialisedTarget { identifier: String },
    /// When there is an attempt to get the value of a target which has not been defined.
    UndefinedTarget { identifier: String },
}

/// An [Environment] represents a set of scopes, stacked on top of one another.
///
/// Note that this is not the same thing as the stack: the stack has a frame for each subroutine call, whereas the environment has a new scope for each block scope (e.g. if-statements, while-loops).
pub struct Environment {
    /// The parent scope.
    parent: Option<MutEnvironment>,
    /// The current scope.
    scope: HashMap<String, Option<Value>>,
}

pub type MutEnvironment = Rc<RefCell<Environment>>;

impl Environment {
    /// Creates a new [Environment].
    pub fn new(parent: Option<MutEnvironment>) -> Self {
        let mut scope = HashMap::new();

        if parent.is_none() {
            [
                ("print", NativeFunction::Print),
                ("format", NativeFunction::Format),
            ]
            .into_iter()
            .for_each(|(identifier, function)| {
                scope.insert(
                    identifier.to_string(),
                    Some(Value::Function(Function::Native(function))),
                );
            });
        }

        Self { scope, parent }
    }

    /// Defines a new target and inserts it into the innermost scope.
    pub fn define(&mut self, identifier: String, value: Option<Value>) {
        self.scope.insert(identifier, value);
    }

    /// Assigns a value to an initialised target.
    ///
    /// In order to find the target to mutate, the program starts in the innermost scope and works outwards until the target is found (or is not found anywhere).
    pub fn assign(
        &mut self,
        identifier: String,
        value: Option<Value>,
    ) -> Result<Option<Value>, EnvironmentError> {
        if let Some(target) = self.scope.get_mut(&identifier) {
            let mut value = value;

            mem::swap(target, &mut value);

            let previous = value;

            Ok(previous)
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(identifier, value)
        } else {
            Err(EnvironmentError::UndefinedAssignmentTarget { identifier })
        }
    }

    /// Gets the value of a target.
    ///
    /// In order to find the target, the program starts in the innermost scope and works outwards until the target is found (or is not found anywhere).
    pub fn get(&self, identifier: &str) -> Result<Value, EnvironmentError> {
        match self.scope.get(identifier) {
            Some(Some(value)) => Ok(value.clone()),
            Some(None) => Err(EnvironmentError::UninitialisedTarget {
                identifier: identifier.to_string(),
            }),
            None => {
                if let Some(parent) = &self.parent {
                    parent.borrow().get(identifier)
                } else {
                    Err(EnvironmentError::UndefinedTarget {
                        identifier: identifier.to_string(),
                    })
                }
            }
        }
    }

    /// Gets the outermost scope.
    ///
    /// Accepts an Rc<RefCell> to itself.
    pub fn global(&self, self_reference: MutEnvironment) -> MutEnvironment {
        if let Some(parent) = &self.parent {
            parent.borrow().global(Rc::clone(parent))
        } else {
            self_reference
        }
    }

    pub fn roots(&self) -> Vec<Pointer> {
        let mut roots = Vec::new();

        for value in self.scope.values() {
            if let Some(Value::ObjectReference(pointer)) = value {
                roots.push(pointer.clone());
            }
        }

        if let Some(parent) = &self.parent {
            roots.append(&mut parent.borrow().roots());
        }

        roots
    }

    pub fn parent(&self) -> Option<MutEnvironment> {
        match &self.parent {
            Some(parent) => Some(Rc::clone(&parent)),
            None => None,
        }
    }

    pub fn values(&self) -> Vec<Value> {
        self.scope
            .iter()
            .filter_map(|(_, value)| value.clone())
            .collect()
    }
}

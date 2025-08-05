use std::collections::HashMap;

use crate::expression::Literal;

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

    pub fn get(&self, identifier: &str) -> Option<Literal> {
        self.variables.get(identifier).cloned()
    }
}

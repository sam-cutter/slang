use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::{Environment, MutEnvironment},
    heap::Pointer,
};

pub struct Stack {
    stack: Vec<MutEnvironment>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: vec![Rc::new(RefCell::new(Environment::new(None)))],
        }
    }

    pub fn top(&mut self) -> MutEnvironment {
        if let Some(top) = self.stack.last() {
            Rc::clone(top)
        } else {
            let top = Rc::new(RefCell::new(Environment::new(None)));

            self.stack.push(Rc::clone(&top));

            top
        }
    }

    pub fn enter_scope(&mut self) {
        if let Some(top) = self.stack.last_mut() {
            *top = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(top)))))
        }
    }

    pub fn exit_scope(&mut self) {
        if let Some(top) = self.stack.last_mut() {
            let parent = if let Some(parent) = top.borrow().parent() {
                parent
            } else {
                return;
            };

            *top = parent;
        }
    }

    pub fn push(&mut self) -> MutEnvironment {
        let global = match self.stack.first() {
            Some(first) => Some(first.borrow().global(Rc::clone(first))),
            None => None,
        };

        let environment = Rc::new(RefCell::new(Environment::new(global)));

        self.stack.push(Rc::clone(&environment));

        environment
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn roots(&self) -> Vec<Pointer> {
        let mut roots = Vec::new();

        for environment in &self.stack {
            roots.append(&mut environment.borrow().roots());
        }

        roots
    }

    pub fn frames_count(&self) -> usize {
        self.stack.len()
    }

    pub fn size(&self) -> usize {
        self.stack.iter().map(|frame| frame.borrow().size()).sum()
    }
}

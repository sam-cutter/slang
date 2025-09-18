use std::{cell::RefCell, rc::Rc};

use crate::{
    heap::{HeapObject, Object, Pointer},
    value::Value,
};

pub struct NaiveHeap {
    heap: Vec<Pointer>,
}

impl NaiveHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn allocate(&mut self, data: Object) -> Pointer {
        let data = data
            .into_iter()
            .map(|(key, value)| match value {
                Value::Object(object) => (key, Value::ObjectReference(self.allocate(object))),
                value => (key, value),
            })
            .collect();

        let heap_object = HeapObject {
            data,
            marked: false,
            reference_count: 1,
        };

        let pointer = Pointer::new(RefCell::new(heap_object));
        self.heap.push(Rc::clone(&pointer));

        pointer
    }

    pub fn objects_count(&self) -> usize {
        self.heap.len()
    }

    pub fn size(&self) -> usize {
        self.heap
            .iter()
            .map(|pointer| size_of_val(&pointer.borrow().data))
            .sum()
    }
}

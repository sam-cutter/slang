use std::{cell::RefCell, rc::Rc};

use crate::{
    heap::{HeapObject, Object, Pointer},
    value::Value,
};

pub struct ReferenceCountedHeap {
    heap: Vec<Pointer>,
}

impl ReferenceCountedHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn allocate(&mut self, data: Object) -> Pointer {
        let data = data
            .into_iter()
            .map(|(key, value)| match value {
                Value::ObjectReference(pointer) => {
                    self.increment(Rc::clone(&pointer));
                    (key, Value::ObjectReference(pointer))
                }
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

    pub fn increment(&mut self, object: Pointer) {
        object.borrow_mut().reference_count += 1;
    }

    pub fn decrement(&mut self, object: Pointer) {
        let count = object.borrow().reference_count;

        match count {
            0 => {
                self.heap
                    .retain(|object| object.borrow().reference_count > 0);
            }
            1 => {
                object.borrow_mut().reference_count -= 1;

                for value in object.borrow().data.values() {
                    if let Value::ObjectReference(pointer) = value {
                        self.decrement(Rc::clone(pointer));
                    }
                }

                self.heap
                    .retain(|object| object.borrow().reference_count > 0);
            }
            2.. => object.borrow_mut().reference_count -= 1,
        }
    }

    pub fn conditionally_decrement(&mut self, value: Value) {
        if let Value::ObjectReference(pointer) = value {
            self.decrement(pointer);
        }
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

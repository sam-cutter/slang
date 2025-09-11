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
        let heap_object = HeapObject {
            data,
            marked: false,
            reference_count: 0,
        };

        let pointer = Pointer::new(RefCell::new(heap_object));
        self.heap.push(Rc::clone(&pointer));
        self.increment(Rc::clone(&pointer));
        // TODO: check correct increment

        pointer
    }

    pub fn increment(&mut self, object: Pointer) {
        object.borrow_mut().reference_count += 1;

        for value in object.borrow().data.values() {
            if let Value::ObjectReference(pointer) = value {
                self.increment(Rc::clone(&pointer));
            }
        }
    }

    pub fn decrement(&mut self, object: Pointer) {
        if object.borrow().reference_count == 0 {
            return;
        }

        object.borrow_mut().reference_count -= 1;

        for value in object.borrow().data.values() {
            if let Value::ObjectReference(pointer) = value {
                self.decrement(Rc::clone(pointer));
            }
        }

        self.heap
            .retain(|object| object.borrow().reference_count > 0);
    }
}

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
        println!("Allocating some data.");
        let heap_object = HeapObject {
            data,
            marked: false,
            reference_count: 0,
        };

        let pointer = Pointer::new(RefCell::new(heap_object));
        self.heap.push(Rc::clone(&pointer));
        self.increment(Rc::clone(&pointer));

        pointer
    }

    pub fn increment(&mut self, object: Pointer) {
        println!("Incrementing something.");
        object.borrow_mut().reference_count += 1;

        for value in object.borrow().data.values() {
            if let Value::ObjectReference(pointer) = value {
                self.increment(Rc::clone(&pointer));
            }
        }
    }

    pub fn decrement(&mut self, object: Pointer) {
        println!(
            "Decrementing something with reference count {}, heap before decrement: {}",
            object.borrow().reference_count,
            self.heap.len()
        );

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

        println!(
            "Finished decrementing, heap after decrement: {}",
            self.heap.len()
        );
    }

    pub fn conditionally_decrement(&mut self, value: Value) {
        match value {
            Value::ObjectReference(pointer) => self.decrement(pointer),
            Value::Object(fields) => {
                for value in fields.values() {
                    if let Value::ObjectReference(pointer) = value {
                        self.decrement(Pointer::clone(pointer));
                    }
                }
            }
            _ => (),
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::heap::{HeapObject, Object, Pointer};

pub struct NaiveHeap {
    heap: Vec<Pointer>,
}

impl NaiveHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn allocate(&mut self, data: Object) -> Pointer {
        let heap_object = HeapObject {
            data,
            marked: false,
            reference_count: 1,
        };

        let pointer = Pointer::new(RefCell::new(heap_object));
        self.heap.push(Rc::clone(&pointer));

        pointer
    }
}

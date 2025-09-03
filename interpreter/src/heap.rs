use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

type Object = HashMap<String, Value>;

pub struct HeapObject {
    pub data: Object,
    pub marked: bool,
}

pub type Pointer = Rc<RefCell<HeapObject>>;

pub struct ManagedHeap {
    heap: Vec<Pointer>,
}

impl ManagedHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn allocate(&mut self, data: Object) -> Pointer {
        let heap_object = HeapObject {
            data,
            marked: false,
        };

        let pointer = Pointer::new(RefCell::new(heap_object));
        self.heap.push(Rc::clone(&pointer));

        pointer
    }

    pub fn manage(&mut self, roots: &[Pointer]) {
        for root in roots {
            self.traverse(Rc::clone(&root));
        }

        self.heap.retain(|value| value.borrow().marked);

        for object in &self.heap {
            object.borrow_mut().marked = false;
        }
    }

    fn traverse(&mut self, root: Pointer) {
        let mut root = root.borrow_mut();
        root.marked = true;

        for value in root.data.values() {
            if let Value::Object(pointer) = value {
                self.traverse(Rc::clone(&pointer));
            }
        }
    }
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    heap::{
        garbage_collected::GarbageCollectedHeap, naive::NaiveHeap,
        reference_counted::ReferenceCountedHeap,
    },
    value::Value,
};

pub mod garbage_collected;
pub mod naive;
pub mod reference_counted;

pub type Object = HashMap<String, Value>;

pub type Pointer = Rc<RefCell<HeapObject>>;

pub struct HeapObject {
    pub data: Object,
    pub marked: bool,
    pub reference_count: usize,
}

pub enum ManagedHeap {
    GarbageCollected(GarbageCollectedHeap),
    Naive(NaiveHeap),
    ReferenceCounted(ReferenceCountedHeap),
}

impl ManagedHeap {
    pub fn allocate(&mut self, data: Object) -> Pointer {
        match self {
            Self::GarbageCollected(heap) => heap.allocate(data),
            Self::Naive(heap) => heap.allocate(data),
            Self::ReferenceCounted(heap) => heap.allocate(data),
        }
    }

    pub fn objects_count(&self) -> usize {
        match self {
            Self::GarbageCollected(heap) => heap.objects_count(),
            Self::Naive(heap) => heap.objects_count(),
            Self::ReferenceCounted(heap) => heap.objects_count(),
        }
    }
}

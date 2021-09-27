use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::values::{Key, Value};

pub struct TableOwner(Rc<RefCell<HashMap<Key, Value>>>);

impl TableOwner {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(HashMap::new())))
    }
}

#[derive(Debug, Clone)]
pub struct Table(Weak<RefCell<HashMap<Key, Value>>>);

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for Table {}

impl Hash for Table {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

impl Table {
    pub fn new(owner: &TableOwner) -> Self {
        Self(Rc::downgrade(&owner.0))
    }

    pub fn insert(&self, key: Key, value: Value) {
        self.0
            .upgrade()
            .expect("table owner was deallocated")
            .borrow_mut()
            .insert(key, value);
    }

    pub fn remove(&self, key: &Key) {
        self.0
            .upgrade()
            .expect("table owner was deallocated")
            .borrow_mut()
            .remove(key);
    }
}

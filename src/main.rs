use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

struct Table(Weak<RefCell<HashMap<Key, Value>>>);

impl Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.upgrade() {
            None => write!(f, "<broken table>"),
            Some(rc) => {
                // This may panic if we're not careful?
                let hash_map = &*rc.borrow();
                hash_map.fmt(f)
            }
        }
    }
}

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

#[derive(PartialEq, Eq, Hash)]
enum Key {
    String(String),
    Bool(bool),
    Int(i64),
    Table(Table),
}

impl Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Bool(b) => b.fmt(f),
            Self::Int(i) => i.fmt(f),
            Self::Table(t) => t.fmt(f),
        }
    }
}

#[derive(PartialEq)]
enum Value {
    Key(Key),
    Float(f64),
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(k) => k.fmt(f),
            Self::Float(d) => d.fmt(f),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut table = HashMap::new();
    table.insert(
        Key::String("Hello".into()),
        Value::Key(Key::String("World".into())),
    );
    let table = Rc::new(RefCell::new(table));

    let table_value = Value::Key(Key::Table(Table(Rc::downgrade(&table))));
    dbg!(table_value);
}

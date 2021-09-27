use std::fmt::Debug;

use crate::builtin::Builtin;
use crate::path::Path;
use crate::table::Table;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Bool(bool),
    Builtin(Builtin),
    Int(i64),
    Nil,
    Path(Path),
    String(Box<String>),
    Table(Table),
}

impl From<Box<String>> for Key {
    fn from(s: Box<String>) -> Self {
        Self::String(s)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self::String(Box::new(s))
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::String(Box::new(s.to_string()))
    }
}

impl From<bool> for Key {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i64> for Key {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<Table> for Key {
    fn from(t: Table) -> Self {
        Self::Table(t)
    }
}

// TODO: Unwrap the Key part so this is only 16, not 24 bytes
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Builtin(Builtin),
    Int(i64),
    Nil,
    Path(Path),
    String(Box<String>),
    Table(Table),

    Float(f64),
}

impl From<Box<String>> for Value {
    fn from(s: Box<String>) -> Self {
        Self::String(s)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(Box::new(s))
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(Box::new(s.to_string()))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<Table> for Value {
    fn from(t: Table) -> Self {
        Self::Table(t)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<Key> for Value {
    fn from(k: Key) -> Self {
        match k {
            Key::Bool(b) => Self::Bool(b),
            Key::Builtin(b) => Self::Builtin(b),
            Key::Int(i) => Self::Int(i),
            Key::Nil => Self::Nil,
            Key::Path(p) => Self::Path(p),
            Key::String(s) => Self::String(s),
            Key::Table(t) => Self::Table(t),
        }
    }
}

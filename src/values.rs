use std::fmt::{self, Debug};

use crate::table::Table;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Key {
    String(Box<String>),
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

#[derive(Clone, PartialEq)]
pub enum Value {
    String(Box<String>),
    Bool(bool),
    Int(i64),
    Float(f64),
    Table(Table),
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Bool(b) => b.fmt(f),
            Self::Int(i) => i.fmt(f),
            Self::Table(t) => t.fmt(f),
            Self::Float(d) => d.fmt(f),
        }
    }
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

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<Table> for Value {
    fn from(t: Table) -> Self {
        Self::Table(t)
    }
}

use std::fmt::Debug;

use crate::table::Table;

// TODO: Unwrap the Key part so this is only 16, not 24 bytes
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Bool(bool),
    // Builtin(Builtin),
    Int(i64),
    // String(Box<String>),
    Table(Table),
    // Float(f64),
    // Nil,
    // Path(Table),
}

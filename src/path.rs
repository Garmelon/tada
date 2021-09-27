use crate::values::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path(Vec<Key>);

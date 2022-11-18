use std::fmt;

use crate::span::{HasSpan, Span};

#[derive(Clone)]
pub struct Space {
    pub comment: Vec<(String, Span)>,
    pub span: Span,
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Space").finish()
    }
}

impl HasSpan for Space {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i#{}", self.name)
    }
}

impl HasSpan for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

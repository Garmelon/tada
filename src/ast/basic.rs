use std::fmt;

use crate::span::{HasSpan, Span};

#[derive(Clone)]
pub enum Line {
    Empty,
    Comment(String),
}

#[derive(Clone)]
pub struct Space {
    pub lines: Vec<Line>,
    pub span: Span,
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lines.iter().any(|l| matches!(l, Line::Comment(_))) {
            write!(f, "space with comments")
        } else {
            write!(f, "space")
        }
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

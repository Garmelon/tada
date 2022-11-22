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

impl Space {
    pub fn empty(span: Span) -> Self {
        Self {
            lines: vec![],
            span,
        }
    }

    pub fn then(mut self, other: Self) -> Self {
        self.lines.extend(other.lines);
        self.span = self.span.join(other.span);
        self
    }

    pub fn then_line(mut self, line: Line) -> Self {
        self.lines.push(line);
        self
    }
}

#[derive(Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new<S: ToString>(name: S, span: Span) -> Self {
        Self {
            name: name.to_string(),
            span,
        }
    }
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

#[derive(Debug, Clone)]
pub struct BoundedSeparated<E> {
    pub elems: Vec<(Space, E, Space)>,
    pub trailing: Option<Space>,
    pub span: Span,
}

impl<E> HasSpan for BoundedSeparated<E> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<E> BoundedSeparated<E> {
    pub fn new(span: Span) -> Self {
        Self {
            elems: vec![],
            trailing: None,
            span,
        }
    }

    pub fn then(mut self, elem: E) -> Self {
        self.elems
            .push((Space::empty(self.span), elem, Space::empty(self.span)));
        self
    }
}

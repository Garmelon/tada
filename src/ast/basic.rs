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

#[derive(Debug, Clone)]
pub enum Separated<E, S1, S2> {
    Empty(Span),
    NonEmpty {
        first_elem: E,
        last_elems: Vec<(S1, E)>,
        trailing: Option<S2>,
        span: Span,
    },
}

impl<E, S1, S2> HasSpan for Separated<E, S1, S2> {
    fn span(&self) -> Span {
        match self {
            Self::Empty(span) => *span,
            Self::NonEmpty { span, .. } => *span,
        }
    }
}

use std::fmt;

use crate::span::{HasSpan, Span};

use super::{TableConstr, TableConstrElem, TableLit, TableLitElem};

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

    pub fn map<E2>(self, f: impl Fn(E) -> E2) -> BoundedSeparated<E2> {
        let elems = self
            .elems
            .into_iter()
            .map(|(s0, e, s1)| (s0, f(e), s1))
            .collect::<Vec<_>>();

        BoundedSeparated {
            elems,
            trailing: self.trailing,
            span: self.span,
        }
    }

    pub fn remove_map<E1, E2>(
        self,
        f: impl Fn(E) -> Result<E1, E2>,
    ) -> (BoundedSeparated<E1>, Vec<(Space, E2, Space)>) {
        let mut kept = vec![];
        let mut removed = vec![];
        for (s0, elem, s1) in self.elems {
            match f(elem) {
                Ok(elem) => kept.push((s0, elem, s1)),
                Err(elem) => removed.push((s0, elem, s1)),
            }
        }
        let new = BoundedSeparated {
            elems: kept,
            trailing: self.trailing,
            span: self.span,
        };
        (new, removed)
    }
}

impl BoundedSeparated<TableLitElem> {
    pub fn table_lit(self) -> TableLit {
        TableLit(self)
    }
}

impl BoundedSeparated<TableConstrElem> {
    pub fn table_constr(self) -> TableConstr {
        TableConstr(self)
    }
}

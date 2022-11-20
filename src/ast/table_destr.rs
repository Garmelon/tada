use crate::span::{HasSpan, Span};

use super::{Expr, Ident, Separated, Space};

// TODO Make table patterns recursive

#[derive(Debug, Clone)]
pub enum TablePatternElem {
    /// `foo`
    Positional(Ident),

    /// `foo: bar`
    ///
    /// Structure: `name s0 : s1 ident`
    Named {
        name: Ident,
        s0: Space,
        s1: Space,
        ident: Ident,
        span: Span,
    },
}

impl HasSpan for TablePatternElem {
    fn span(&self) -> Span {
        match self {
            TablePatternElem::Positional(ident) => ident.span(),
            TablePatternElem::Named { span, .. } => *span,
        }
    }
}

/// `{ foo, bar: baz }`
///
/// Structure: `{ s0 elems s1 }`
#[derive(Debug, Clone)]
pub struct TablePattern {
    pub s0: Space,
    pub elems: Separated<TablePatternElem, (Space, Space), Space>,
    pub s1: Space,
    pub span: Span,
}

impl HasSpan for TablePattern {
    fn span(&self) -> Span {
        self.span
    }
}

/// - `{ foo, bar: baz } = a`
/// - `local { foo, bar: baz } = a`
///
/// Structure: `local pattern s0 = s1 value`
#[derive(Debug, Clone)]
pub struct TableDestr {
    pub local: Option<Space>,
    pub pattern: TablePattern,
    pub s0: Space,
    pub s1: Space,
    pub value: Box<Expr>,
    pub span: Span,
}

impl HasSpan for TableDestr {
    fn span(&self) -> Span {
        self.span
    }
}

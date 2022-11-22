use crate::span::{HasSpan, Span};

use super::{Expr, Ident, Space};

#[derive(Debug, Clone)]
pub enum Field {
    /// `a[b]`
    ///
    /// Structure: `expr s0 [ s1 index s2 ]`
    Access {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        span: Span,
    },

    /// `a[b] = c`
    ///
    /// Structure: `expr s0 [ s1 index s2 ] s3 = s4 value`
    Assign {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        s3: Space,
        s4: Space,
        value: Box<Expr>,
        span: Span,
    },

    /// `a.foo`
    ///
    /// Structure: `expr s0 . s1 ident`
    AccessIdent {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        ident: Ident,
        span: Span,
    },

    /// `a.foo = b`
    ///
    /// Structure: `expr s0 . s1 ident s2 = s3 value`
    AssignIdent {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        ident: Ident,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for Field {
    fn span(&self) -> Span {
        match self {
            Self::Access { span, .. } => *span,
            Self::Assign { span, .. } => *span,
            Self::AccessIdent { span, .. } => *span,
            Self::AssignIdent { span, .. } => *span,
        }
    }
}

impl Field {
    pub fn access(base: Box<Expr>, index: Box<Expr>, span: Span) -> Self {
        Self::Access {
            expr: base,
            s0: Space::empty(span),
            s1: Space::empty(span),
            index,
            s2: Space::empty(span),
            span,
        }
    }

    pub fn assign(base: Box<Expr>, index: Box<Expr>, value: Box<Expr>, span: Span) -> Self {
        Self::Assign {
            expr: base,
            s0: Space::empty(span),
            s1: Space::empty(span),
            index,
            s2: Space::empty(span),
            s3: Space::empty(span),
            s4: Space::empty(span),
            value,
            span,
        }
    }

    pub fn expr(self) -> Expr {
        Expr::Field(self)
    }
}

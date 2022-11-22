use crate::span::{HasSpan, Span};

use super::{Expr, Ident, Space};

#[derive(Debug, Clone)]
pub enum Var {
    /// `[a]`
    ///
    /// Structure: `[ s0 index s1 ]`
    Access {
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        span: Span,
    },

    /// - `[a] = b`
    /// - `local [a] = b`
    ///
    /// Structure: `local [ s0 index s1 ] s2 = s3 value`
    Assign {
        local: Option<Space>,
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
        span: Span,
    },

    /// `foo`
    AccessIdent(Ident),

    /// - `foo = a`
    /// - `local foo = a`
    ///
    /// Structure: `local name s0 = s1 value`
    AssignIdent {
        local: Option<Space>,
        name: Ident,
        s0: Space,
        s1: Space,
        value: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for Var {
    fn span(&self) -> Span {
        match self {
            Self::Access { span, .. } => *span,
            Self::Assign { span, .. } => *span,
            Self::AccessIdent(ident) => ident.span(),
            Self::AssignIdent { span, .. } => *span,
        }
    }
}

impl Var {
    pub fn access(index: Box<Expr>, span: Span) -> Self {
        Self::Access {
            s0: Space::empty(span),
            index,
            s1: Space::empty(span),
            span,
        }
    }

    pub fn assign(local: bool, index: Box<Expr>, value: Box<Expr>, span: Span) -> Self {
        let local = if local {
            Some(Space::empty(span))
        } else {
            None
        };

        Self::Assign {
            local,
            s0: Space::empty(span),
            index,
            s1: Space::empty(span),
            s2: Space::empty(span),
            s3: Space::empty(span),
            value,
            span,
        }
    }

    pub fn assign_ident(local: bool, name: Ident, value: Box<Expr>, span: Span) -> Self {
        let local = if local {
            Some(Space::empty(span))
        } else {
            None
        };

        Self::AssignIdent {
            local,
            name,
            s0: Space::empty(span),
            s1: Space::empty(span),
            value,
            span,
        }
    }

    pub fn expr(self) -> Expr {
        Expr::Var(self)
    }
}

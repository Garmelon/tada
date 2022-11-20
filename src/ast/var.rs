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

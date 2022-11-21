use crate::span::{HasSpan, Span};

use super::{BoundedSeparated, Expr, Space, TableLitElem};

#[derive(Debug, Clone)]
pub enum Program {
    /// Structure: `s0 lit s1`
    Expr {
        s0: Space,
        expr: Expr,
        s1: Space,
        span: Span,
    },

    /// Structure: `s0 module elems`
    Module {
        s0: Space,
        elems: BoundedSeparated<TableLitElem>,
        span: Span,
    },
}

impl HasSpan for Program {
    fn span(&self) -> Span {
        match self {
            Self::Expr { span, .. } => *span,
            Self::Module { span, .. } => *span,
        }
    }
}

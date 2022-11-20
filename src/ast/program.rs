use crate::span::{HasSpan, Span};

use super::{Expr, Space, TableLitElem};

#[derive(Debug, Clone)]
pub enum Program {
    /// Structure: `s0 lit s1`
    Expr {
        s0: Space,
        expr: Expr,
        s1: Space,
        span: Span,
    },

    /// Structure: `s0 module elems trailing_comma`
    Module {
        s0: Space,
        elems: Vec<(Space, TableLitElem, Space)>,
        /// `Some` if there is a trailing comma, `None` otherwise.
        trailing_comma: Option<Space>,
        span: Span,
    },
}

impl HasSpan for Program {
    fn span(&self) -> Span {
        match self {
            Program::Expr { span, .. } => *span,
            Program::Module { span, .. } => *span,
        }
    }
}

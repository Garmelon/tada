use crate::span::{HasSpan, Span};

use super::{Expr, Separated, Space, TableLitElem};

#[derive(Debug, Clone)]
pub enum Program {
    /// Structure: `s0 lit s1`
    Expr {
        s0: Space,
        expr: Expr,
        s1: Space,
        span: Span,
    },

    /// Structure: `s0 module s1 elems s2`
    Module {
        s0: Space,
        s1: Space,
        elems: Separated<TableLitElem, (Space, Space), Space>,
        s2: Space,
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

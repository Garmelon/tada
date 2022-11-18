use crate::span::{HasSpan, Span};

use super::basic::Space;
use super::expr::Expr;
use super::table_constr::TableConstr;

#[derive(Debug, Clone)]
pub enum Call {
    /// `a(b)`
    ///
    /// Structure: `expr s0 ( s1 arg s2)
    Arg {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        arg: Box<Expr>,
        s2: Space,
        span: Span,
    },

    /// `a()`
    ///
    /// Structure: `expr s0 ( s1 )`
    NoArg {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        span: Span,
    },

    /// `a{..}`
    ///
    /// Structure: `expr s0 constr`
    Constr {
        expr: Box<Expr>,
        s0: Space,
        constr: TableConstr,
        span: Span,
    },
}

impl HasSpan for Call {
    fn span(&self) -> Span {
        match self {
            Call::Arg { span, .. } => *span,
            Call::NoArg { span, .. } => *span,
            Call::Constr { span, .. } => *span,
        }
    }
}

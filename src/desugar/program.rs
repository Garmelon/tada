use crate::ast::{Program, Space};

impl Program {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Expr { s0, expr, s1, span } => {
                let (expr, desugared) = expr.desugar();
                let new = Self::Expr { s0, expr, s1, span };
                (new, desugared)
            }

            Self::Module { s0, elems, span } => {
                // `s0 module elems`
                // -> `s0 table`
                let new = Self::Expr {
                    s0,
                    expr: elems.table_lit().lit().expr(),
                    s1: Space::empty(span),
                    span,
                };
                (new, true)
            }
        }
    }
}

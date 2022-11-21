use crate::ast::{Expr, Lit, Program, Space, TableLit};

impl Program {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Expr { s0, expr, s1, span } => {
                let (expr, desugared) = expr.desugar();
                let new = Self::Expr { s0, expr, s1, span };
                (new, desugared)
            }

            Self::Module {
                s0,
                s1,
                elems,
                s2,
                span,
            } => {
                // `s0 module s1 elems s2`
                // -> `s0 '{ s1 elems s2 } empty`
                let table = TableLit {
                    s0: s1,
                    elems,
                    s1: s2,
                    span,
                };
                let new = Self::Expr {
                    s0,
                    expr: Expr::Lit(Lit::Table(table)),
                    s1: Space::empty(span),
                    span,
                };
                (new, true)
            }
        }
    }
}

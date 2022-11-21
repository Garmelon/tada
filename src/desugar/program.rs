use crate::ast::{Expr, Lit, Program, Space, TableLit};

impl Program {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Expr { s0, expr, s1, span } => {
                let (expr, desugared) = expr.desugar();
                (Self::Expr { s0, expr, s1, span }, desugared)
            }

            Self::Module {
                s0,
                s1,
                elems,
                s2,
                span,
            } => {
                let (elems, desugared) = elems.desugar_elem(|e| e.desugar());
                if desugared {
                    let new = Self::Module {
                        s0,
                        s1,
                        elems,
                        s2,
                        span,
                    };
                    return (new, true);
                }

                let table = TableLit {
                    s0: s1,
                    elems,
                    s1: Space::empty(span),
                    span,
                };
                let new = Self::Expr {
                    s0,
                    expr: Expr::Lit(Lit::Table(table)),
                    s1: s2,
                    span,
                };
                (new, true)
            }
        }
    }
}

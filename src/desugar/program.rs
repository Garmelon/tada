use crate::ast::{Expr, Lit, Program, Space, TableLit};
use crate::span::HasSpan;

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
                let (elems, desugared) = elems.desugar_elem(|e| (e, false)); // TODO Implement
                if desugared {
                    let new = Self::Module {
                        s0,
                        s1,
                        elems,
                        s2,
                        span,
                    };
                    (new, true)
                } else {
                    let elems_span = elems.span();
                    let table = TableLit {
                        s0: s1,
                        elems,
                        s1: Space::empty(elems_span.at_end()),
                        span: elems_span,
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
}

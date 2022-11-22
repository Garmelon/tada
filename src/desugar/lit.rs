use crate::ast::{Expr, Lit, TableLit, TableLitElem};

impl TableLitElem {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Positional(expr) => {
                let (expr, desugared) = expr.desugar();
                (Self::Positional(expr.boxed()), desugared)
            }

            Self::Named {
                name,
                s0,
                s1,
                value,
                span,
            } => {
                let (value, desugared) = value.desugar();
                let new = Self::Named {
                    name,
                    s0,
                    s1,
                    value: value.boxed(),
                    span,
                };
                (new, desugared)
            }
        }
    }
}

impl TableLit {
    pub fn desugar(self) -> (Self, bool) {
        let (elems, desugared) = self.0.desugar(|e| e.desugar());
        if desugared {
            (elems.table_lit(), true)
        } else {
            let (elems, removed) = elems.remove_map(|e| match e {
                TableLitElem::Named { value, .. } if matches!(*value, Expr::Lit(Lit::Nil(_))) => {
                    Err(())
                }
                e => Ok(e),
            });
            (elems.table_lit(), !removed.is_empty())
        }
    }
}

impl Lit {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Table(table) => {
                let (table, desugared) = table.desugar();
                (table.lit(), desugared)
            }

            lit => (lit, false),
        }
    }
}

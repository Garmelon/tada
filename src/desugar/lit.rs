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
        let (elems, removed) = self.0.remove_map(|e| match e {
            TableLitElem::Named { value, .. } if matches!(*value, Expr::Lit(Lit::Nil(_))) => {
                Err(())
            }
            e => Ok(e),
        });
        if removed.is_empty() {
            let (elems, desugared) = elems.desugar(|e| e.desugar());
            (elems.table_lit(), desugared)
        } else {
            (elems.table_lit(), true)
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

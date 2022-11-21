use crate::ast::{Lit, TableLit, TableLitElem};

impl TableLitElem {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Positional(expr) => {
                let (expr, desugared) = expr.desugar();
                (Self::Positional(Box::new(expr)), desugared)
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
                    value: Box::new(value),
                    span,
                };
                (new, desugared)
            }
        }
    }
}

impl TableLit {
    pub fn desugar(self) -> (Self, bool) {
        let Self {
            s0,
            elems,
            s1,
            span,
        } = self;

        let (elems, desugared) = elems.desugar_elem(|e| e.desugar());
        let new = Self {
            s0,
            elems,
            s1,
            span,
        };
        (new, desugared)
    }
}

impl Lit {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Table(table) => {
                let (table, desugared) = table.desugar();
                (Self::Table(table), desugared)
            }

            lit => (lit, false),
        }
    }
}

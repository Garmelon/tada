use crate::ast::{
    BoundedSeparated, Call, Expr, Ident, Lit, Space, StringLit, TableConstr, TableConstrElem,
    TableDestr, TableLitElem, TablePattern, TablePatternElem,
};
use crate::builtin::Builtin;

fn pattern_to_constr(pattern: TablePattern) -> TableConstr {
    TableConstr(pattern.0.map(|e| match e {
        TablePatternElem::Positional(ident) => TableConstrElem::Lit(TableLitElem::Positional(
            Box::new(Expr::Lit(Lit::String(StringLit::from_ident(ident)))),
        )),

        TablePatternElem::Named {
            name,
            s0,
            s1,
            ident,
            span,
        } => TableConstrElem::Lit(TableLitElem::Named {
            name,
            s0,
            s1,
            value: Box::new(Expr::Lit(Lit::String(StringLit::from_ident(ident)))),
            span,
        }),
    }))
}

impl TableDestr {
    pub fn desugar(self) -> (Expr, bool) {
        let Self {
            local,
            pattern,
            s0: _,
            s1: _,
            value,
            span,
        } = self;

        let mut constr = BoundedSeparated::new(span)
            .then(TableConstrElem::Lit(TableLitElem::Positional(Box::new(
                Expr::TableConstr(pattern_to_constr(pattern)),
            ))))
            .then(TableConstrElem::Lit(TableLitElem::Positional(value)));
        if local.is_some() {
            constr = constr.then(TableConstrElem::Lit(TableLitElem::Named {
                name: Ident::new("local", span),
                s0: Space::empty(span),
                s1: Space::empty(span),
                value: Box::new(Expr::Lit(Lit::Bool(true, span))),
                span,
            }));
        }

        let new = Expr::Call(Call::Constr {
            expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Destructure, span))),
            s0: Space::empty(span),
            constr: TableConstr(constr),
            span,
        });
        (new, true)
    }
}

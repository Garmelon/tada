use crate::ast::{
    BoundedSeparated, Call, Expr, Ident, Lit, Space, StringLit, TableConstr, TableConstrElem,
    TableDestr, TableLitElem, TablePattern, TablePatternElem,
};
use crate::builtin::Builtin;

fn pattern_to_constr(pattern: TablePattern) -> TableConstr {
    pattern
        .0
        .map(|e| match e {
            TablePatternElem::Positional(ident) => TableConstrElem::Lit(TableLitElem::Positional(
                StringLit::from_ident(ident).lit().expr().boxed(),
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
                value: StringLit::from_ident(ident).lit().expr().boxed(),
                span,
            }),
        })
        .table_constr()
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
            .then(TableConstrElem::Lit(TableLitElem::Positional(
                pattern_to_constr(pattern).expr().boxed(),
            )))
            .then(TableConstrElem::Lit(TableLitElem::Positional(value)));
        if local.is_some() {
            constr = constr.then(TableConstrElem::Lit(TableLitElem::named(
                Ident::new("local", span),
                Lit::Bool(true, span).expr().boxed(),
                span,
            )));
        }

        let new = Call::Constr {
            expr: Lit::Builtin(Builtin::Destructure, span).expr().boxed(),
            s0: Space::empty(span),
            constr: constr.table_constr(),
            span,
        };
        (new.expr(), true)
    }
}

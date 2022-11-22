use crate::ast::{BoundedSeparated, Call, Expr, Field, Lit, Space, StringLit, TableConstrElem};
use crate::builtin::Builtin;

impl Field {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                expr,
                s0: _,
                s1: _,
                index,
                s2: _,
                span,
            } => {
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(expr))
                    .then(TableConstrElem::positional(index))
                    .table_constr();
                let new = Call::Constr {
                    expr: Lit::Builtin(Builtin::Get, span).expr().boxed(),
                    s0: Space::empty(span),
                    constr,
                    span,
                };
                (new.expr(), true)
            }

            Self::Assign {
                expr,
                s0: _,
                s1: _,
                index,
                s2: _,
                s3: _,
                s4: _,
                value,
                span,
            } => {
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(expr))
                    .then(TableConstrElem::positional(index))
                    .then(TableConstrElem::positional(value))
                    .table_constr();
                let new = Call::Constr {
                    expr: Lit::Builtin(Builtin::Set, span).expr().boxed(),
                    s0: Space::empty(span),
                    constr,
                    span,
                };
                (new.expr(), true)
            }

            Self::AccessIdent {
                expr,
                s0,
                s1,
                ident,
                span,
            } => {
                // `expr s0 . s1 ident´
                // -> `expr s0 [ s1 ident_str ]`
                let new = Self::Access {
                    expr,
                    s0,
                    s1,
                    index: StringLit::from_ident(ident).lit().expr().boxed(),
                    s2: Space::empty(span),
                    span,
                };
                (new.expr(), true)
            }

            Self::AssignIdent {
                expr,
                s0,
                s1,
                ident,
                s2,
                s3,
                value,
                span,
            } => {
                // `expr s0 . s1 ident s2 = s3 value`
                // -> `expr s0 [ s1 ident_str ] s2 = s3 value`
                let new = Self::Assign {
                    expr,
                    s0,
                    s1,
                    index: StringLit::from_ident(ident).lit().expr().boxed(),
                    s2: Space::empty(span),
                    s3: s2,
                    s4: s3,
                    value,
                    span,
                };
                (new.expr(), true)
            }
        }
    }
}

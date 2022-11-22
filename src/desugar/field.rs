use crate::ast::{BoundedSeparated, Call, Expr, Field, Lit, StringLit, TableConstrElem};
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
                let new = Call::constr(
                    Lit::Builtin(Builtin::Get, span).expr().boxed(),
                    constr,
                    span,
                );
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
                let new = Call::constr(
                    Lit::Builtin(Builtin::Set, span).expr().boxed(),
                    constr,
                    span,
                );
                (new.expr(), true)
            }

            Self::AccessIdent {
                expr,
                s0: _,
                s1: _,
                ident,
                span,
            } => {
                let new = Self::access(
                    expr,
                    StringLit::from_ident(ident).lit().expr().boxed(),
                    span,
                );
                (new.expr(), true)
            }

            Self::AssignIdent {
                expr,
                s0: _,
                s1: _,
                ident,
                s2: _,
                s3: _,
                value,
                span,
            } => {
                let new = Self::assign(
                    expr,
                    StringLit::from_ident(ident).lit().expr().boxed(),
                    value,
                    span,
                );
                (new.expr(), true)
            }
        }
    }
}

use crate::ast::{BoundedSeparated, Call, Expr, Field, Lit, StringLit, TableConstrElem, Var};
use crate::builtin::Builtin;
use crate::span::HasSpan;

impl Var {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                s0: _,
                index,
                s1: _,
                span,
            } => {
                let scope = Call::no_arg(Lit::Builtin(Builtin::Scope, span).expr().boxed(), span);
                let new = Field::access(scope.expr().boxed(), index, span);
                (new.expr(), true)
            }

            Self::Assign {
                local: None,
                s0: _,
                index,
                s1: _,
                s2: _,
                s3: _,
                value,
                span,
            } => {
                let scope = Call::no_arg(Lit::Builtin(Builtin::Scope, span).expr().boxed(), span);
                let new = Field::assign(scope.expr().boxed(), index, value, span);
                (new.expr(), true)
            }

            Self::Assign {
                local: Some(_),
                s0: _,
                index,
                s1: _,
                s2: _,
                s3: _,
                value,
                span,
            } => {
                let scope = Call::no_arg(Lit::Builtin(Builtin::Scope, span).expr().boxed(), span);
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(scope.expr().boxed()))
                    .then(TableConstrElem::positional(index))
                    .then(TableConstrElem::positional(value))
                    .table_constr();
                let new = Call::constr(
                    Lit::Builtin(Builtin::SetRaw, span).expr().boxed(),
                    constr,
                    span,
                );
                (new.expr(), true)
            }

            Self::AccessIdent(name) => {
                let span = name.span();
                let new = Self::access(StringLit::from_ident(name).lit().expr().boxed(), span);
                (new.expr(), true)
            }

            Self::AssignIdent {
                local,
                name,
                s0: _,
                s1: _,
                value,
                span,
            } => {
                let new = Self::assign(
                    local.is_some(),
                    StringLit::from_ident(name).lit().expr().boxed(),
                    value,
                    span,
                );
                (new.expr(), true)
            }
        }
    }
}

use crate::ast::{
    BoundedSeparated, Call, Expr, Field, Lit, Space, StringLit, TableConstrElem, Var,
};
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
                // `name`
                // -> `[ name_str ]`
                let span = name.span();
                let new = Self::Access {
                    s0: Space::empty(span),
                    index: StringLit::from_ident(name).lit().expr().boxed(),
                    s1: Space::empty(span),
                    span,
                };
                (new.expr(), true)
            }

            Self::AssignIdent {
                local,
                name,
                s0,
                s1,
                value,
                span,
            } => {
                // `local name s0 = s1 value`
                // -> `local [ name_str ] s0 = s1 value`
                let new = Self::Assign {
                    local,
                    s0: Space::empty(span),
                    index: StringLit::from_ident(name).lit().expr().boxed(),
                    s1: Space::empty(span),
                    s2: s0,
                    s3: s1,
                    value,
                    span,
                };
                (new.expr(), true)
            }
        }
    }
}

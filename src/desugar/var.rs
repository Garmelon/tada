use crate::ast::{
    BoundedSeparated, Call, Expr, Field, Lit, Space, StringLit, TableConstrElem, Var,
};
use crate::builtin::Builtin;
use crate::span::HasSpan;

impl Var {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                s0,
                index,
                s1,
                span,
            } => {
                // `[ s0 index s1 ]`
                // -> `'scope()[ s0 index s1 ]`
                let scope = Call::NoArg {
                    expr: Lit::Builtin(Builtin::Scope, span).expr().boxed(),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                };
                let new = Field::Access {
                    expr: scope.expr().boxed(),
                    s0: Space::empty(span),
                    s1: s0,
                    index,
                    s2: s1,
                    span,
                };
                (new.expr(), true)
            }

            Self::Assign {
                local: None,
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span,
            } => {
                // `[ s0 index s1 ] s2 = s3 value`
                // -> `'scope()[ s0 index s1 ] s2 = s3 value`
                let scope = Call::NoArg {
                    expr: Lit::Builtin(Builtin::Scope, span).expr().boxed(),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                };
                let new = Field::Assign {
                    expr: scope.expr().boxed(),
                    s0: Space::empty(span),
                    s1: s0,
                    index,
                    s2: s1,
                    s3: s2,
                    s4: s3,
                    value,
                    span,
                };
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
                let scope = Call::NoArg {
                    expr: Lit::Builtin(Builtin::Scope, span).expr().boxed(),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                };
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(scope.expr().boxed()))
                    .then(TableConstrElem::positional(index))
                    .then(TableConstrElem::positional(value))
                    .table_constr();
                let new = Call::Constr {
                    expr: Lit::Builtin(Builtin::SetRaw, span).expr().boxed(),
                    s0: Space::empty(span),
                    constr,
                    span,
                };
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

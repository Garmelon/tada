use crate::ast::{
    BoundedSeparated, Call, Expr, FuncDef, Ident, Lit, Space, TableConstrElem, TableLitElem, Var,
};
use crate::builtin::Builtin;

impl FuncDef {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::AnonNoArg {
                s0: _,
                s1: _,
                s2: _,
                body,
                span,
            } => {
                let quote = BoundedSeparated::new(span)
                    .then(TableLitElem::Named {
                        name: Ident::new("quote", span),
                        s0: Space::empty(span),
                        s1: Space::empty(span),
                        value: body,
                        span,
                    })
                    .table_lit();
                let quote = quote.lit().expr().boxed();
                let scope = Call::NoArg {
                    expr: Lit::Builtin(Builtin::Scope, span).expr().boxed(),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                };
                let new = BoundedSeparated::new(span)
                    .then(TableConstrElem::Lit(TableLitElem::Positional(quote)))
                    .then(TableConstrElem::Lit(TableLitElem::Named {
                        name: Ident::new("scope", span),
                        s0: Space::empty(span),
                        s1: Space::empty(span),
                        value: scope.expr().boxed(),
                        span,
                    }))
                    .table_constr()
                    .expr();
                (new, true)
            }

            Self::AnonArg {
                s0: _,
                s1: _,
                arg,
                s2: _,
                s3: _,
                body,
                span,
            } => {
                // `function s0 ( s1 arg s2 ) s3 body`
                // -> `function ( ) '{ local arg = 'arg(), body }`
                let arg_call = Call::NoArg {
                    expr: Lit::Builtin(Builtin::Arg, span).expr().boxed(),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                };
                let arg_assign = Var::AssignIdent {
                    local: Some(Space::empty(span)),
                    name: arg,
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    value: arg_call.expr().boxed(),
                    span,
                };
                let body = BoundedSeparated::new(span)
                    .then(TableLitElem::Positional(arg_assign.expr().boxed()))
                    .then(TableLitElem::Positional(body))
                    .table_lit();
                let new = Self::AnonNoArg {
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    s2: Space::empty(span),
                    body: body.lit().expr().boxed(),
                    span,
                };
                (new.expr(), true)
            }

            Self::AnonDestr {
                s0,
                pattern,
                s1,
                body,
                span,
            } => todo!(),

            Self::NamedNoArg {
                local,
                s0,
                name,
                s1,
                s2,
                s3,
                body,
                span,
            } => todo!(),

            Self::NamedArg {
                local,
                s0,
                name,
                s1,
                s2,
                arg,
                s3,
                s4,
                body,
                span,
            } => todo!(),

            Self::NamedDestr {
                local,
                s0,
                name,
                s1,
                pattern,
                s2,
                body,
                span,
            } => todo!(),
        }
    }
}

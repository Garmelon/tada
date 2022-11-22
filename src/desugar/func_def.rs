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
                    .then(TableLitElem::named(Ident::new("quote", span), body, span))
                    .table_lit();
                let scope = Call::no_arg(Lit::Builtin(Builtin::Scope, span).expr().boxed(), span);
                let new = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(Box::new(quote.lit().expr())))
                    .then(TableConstrElem::named(
                        Ident::new("scope", span),
                        scope.expr().boxed(),
                        span,
                    ))
                    .table_constr();
                (new.expr(), true)
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
                let arg_call = Call::no_arg(Lit::Builtin(Builtin::Arg, span).expr().boxed(), span);
                let arg_assign = Var::assign_ident(true, arg, arg_call.expr().boxed(), span);
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

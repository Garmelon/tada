use crate::ast::{
    BoundedSeparated, Call, Expr, FuncDef, Ident, Lit, Space, TableConstr, TableConstrElem,
    TableLit, TableLitElem, Var,
};
use crate::builtin::Builtin;

impl FuncDef {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::AnonNoArg {
                s0,
                s1,
                s2,
                body,
                span,
            } => {
                // `function s0 ( s1 ) s2 body`
                // -> `{ '{ quote: body }, scope: 'scope() }`
                let quote = Expr::Lit(Lit::Table(TableLit(BoundedSeparated {
                    elems: vec![(
                        Space::empty(span),
                        TableLitElem::Named {
                            name: Ident::new("quote", span),
                            s0: Space::empty(span),
                            s1: Space::empty(span),
                            value: body,
                            span,
                        },
                        Space::empty(span),
                    )],
                    trailing: None,
                    span,
                })));
                let scope = Expr::Call(Call::NoArg {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Scope, span))),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                });
                let new = Expr::TableConstr(TableConstr(BoundedSeparated {
                    elems: vec![
                        (
                            Space::empty(span),
                            TableConstrElem::Lit(TableLitElem::Positional(Box::new(quote))),
                            Space::empty(span),
                        ),
                        (
                            Space::empty(span),
                            TableConstrElem::Lit(TableLitElem::Named {
                                name: Ident::new("scope", span),
                                s0: Space::empty(span),
                                s1: Space::empty(span),
                                value: Box::new(scope),
                                span,
                            }),
                            Space::empty(span),
                        ),
                    ],
                    trailing: None,
                    span,
                }));
                (new, true)
            }

            Self::AnonArg {
                s0,
                s1,
                arg,
                s2,
                s3,
                body,
                span,
            } => {
                // `function s0 ( s1 arg s2 ) s3 body`
                // -> `function ( ) '{ local arg = 'arg(), body }`
                let arg_call = Expr::Call(Call::NoArg {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Arg, span))),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                });
                let arg_assign = Expr::Var(Var::AssignIdent {
                    local: Some(Space::empty(span)),
                    name: arg,
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    value: Box::new(arg_call),
                    span,
                });
                let body_elems = vec![
                    (
                        Space::empty(span),
                        TableLitElem::Positional(Box::new(arg_assign)),
                        Space::empty(span),
                    ),
                    (
                        Space::empty(span),
                        TableLitElem::Positional(body),
                        Space::empty(span),
                    ),
                ];
                let body = Expr::Lit(Lit::Table(TableLit(BoundedSeparated {
                    elems: body_elems,
                    trailing: None,
                    span,
                })));
                let new = Expr::FuncDef(Self::AnonNoArg {
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    s2: Space::empty(span),
                    body: Box::new(body),
                    span,
                });
                (new, true)
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

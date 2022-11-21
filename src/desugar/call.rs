use crate::ast::{BoundedSeparated, Call, Expr, Ident, Lit, Space, TableLit, TableLitElem};

// TODO Add span for just the parentheses to ast, or limit span to parentheses

impl Call {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Arg {
                expr,
                s0,
                s1,
                arg,
                s2,
                span,
            } => {
                // `expr s0 ( s1 arg s2 )`
                // -> `'{ s0 call: expr, arg: s1 arg s2 }`
                let call = TableLitElem::Named {
                    name: Ident::new("call", span),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    value: expr,
                    span,
                };
                let arg = TableLitElem::Named {
                    name: Ident::new("arg", span),
                    s0: Space::empty(span),
                    s1,
                    value: arg,
                    span,
                };
                let elems = vec![
                    (s0, call, Space::empty(span)),
                    (Space::empty(span), arg, s2),
                ];
                let new = Expr::Lit(Lit::Table(TableLit(BoundedSeparated {
                    elems,
                    trailing: None,
                    span,
                })));
                (new, true)
            }

            Self::NoArg { expr, s0, s1, span } => {
                // `expr s0 ( s1 )`
                // -> `expr s0 ( s1 nil )`
                let new = Expr::Call(Self::Arg {
                    expr,
                    s0,
                    s1,
                    arg: Box::new(Expr::Lit(Lit::Nil(span))),
                    s2: Space::empty(span),
                    span,
                });
                (new, true)
            }

            Self::Constr {
                expr,
                s0,
                constr,
                span,
            } => {
                // `expr s0 {..}`
                // -> `expr s0 ( {..} )`
                let new = Expr::Call(Self::Arg {
                    expr,
                    s0,
                    s1: Space::empty(span),
                    arg: Box::new(Expr::TableConstr(constr)),
                    s2: Space::empty(span),
                    span,
                });
                (new, true)
            }
        }
    }
}

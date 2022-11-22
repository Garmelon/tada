use crate::ast::{BoundedSeparated, Call, Expr, Ident, Lit, Space, TableLitElem};

// TODO Add span for just the parentheses to ast, or limit span to parentheses

impl Call {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Arg {
                expr,
                s0: _,
                s1: _,
                arg,
                s2: _,
                span,
            } => {
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
                    s1: Space::empty(span),
                    value: arg,
                    span,
                };
                let new = Expr::Lit(Lit::Table(
                    BoundedSeparated::new(span).then(call).then(arg).table_lit(),
                ));
                (new, true)
            }

            Self::NoArg { expr, s0, s1, span } => {
                let new = Expr::Call(Self::Arg {
                    expr,
                    s0,
                    s1,
                    arg: Expr::Lit(Lit::Nil(span)).boxed(),
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
                let new = Expr::Call(Self::Arg {
                    expr,
                    s0,
                    s1: Space::empty(span),
                    arg: Expr::TableConstr(constr).boxed(),
                    s2: Space::empty(span),
                    span,
                });
                (new, true)
            }
        }
    }
}

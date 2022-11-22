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
                let new = BoundedSeparated::new(span)
                    .then(TableLitElem::named(Ident::new("call", span), expr, span))
                    .then(TableLitElem::named(Ident::new("arg", span), arg, span))
                    .table_lit()
                    .lit()
                    .expr();
                (new, true)
            }

            Self::NoArg { expr, s0, s1, span } => {
                let new = Self::Arg {
                    expr,
                    s0,
                    s1,
                    arg: Lit::Nil(span).expr().boxed(),
                    s2: Space::empty(span),
                    span,
                };
                (new.expr(), true)
            }

            Self::Constr {
                expr,
                s0,
                constr,
                span,
            } => {
                let new = Self::Arg {
                    expr,
                    s0,
                    s1: Space::empty(span),
                    arg: constr.expr().boxed(),
                    s2: Space::empty(span),
                    span,
                };
                (new.expr(), true)
            }
        }
    }
}

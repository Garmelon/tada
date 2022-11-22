use crate::ast::{BoundedSeparated, Call, Expr, Ident, Lit, TableLitElem};

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
                    .table_lit();
                (new.lit().expr(), true)
            }

            Self::NoArg {
                expr,
                s0: _,
                s1: _,
                span,
            } => {
                let new = Self::arg(expr, Lit::Nil(span).expr().boxed(), span);
                (new.expr(), true)
            }

            Self::Constr {
                expr,
                s0: _,
                constr,
                span,
            } => {
                let new = Self::arg(expr, constr.expr().boxed(), span);
                (new.expr(), true)
            }
        }
    }
}

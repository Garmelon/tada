use crate::ast::{
    BoundedSeparated, Call, Expr, Field, Lit, Space, StringLit, TableConstrElem, TableLitElem,
};
use crate::builtin::Builtin;

impl Field {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                expr,
                s0: _,
                s1: _,
                index,
                s2: _,
                span,
            } => {
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::Lit(TableLitElem::Positional(expr)))
                    .then(TableConstrElem::Lit(TableLitElem::Positional(index)))
                    .table_constr();
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Get, span))),
                    s0: Space::empty(span),
                    constr,
                    span,
                });
                (new, true)
            }

            Self::Assign {
                expr,
                s0: _,
                s1: _,
                index,
                s2: _,
                s3: _,
                s4: _,
                value,
                span,
            } => {
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::Lit(TableLitElem::Positional(expr)))
                    .then(TableConstrElem::Lit(TableLitElem::Positional(index)))
                    .then(TableConstrElem::Lit(TableLitElem::Positional(value)))
                    .table_constr();
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Set, span))),
                    s0: Space::empty(span),
                    constr,
                    span,
                });
                (new, true)
            }

            Self::AccessIdent {
                expr,
                s0,
                s1,
                ident,
                span,
            } => {
                // `expr s0 . s1 ident´
                // -> `expr s0 [ s1 ident_str ]`
                let new = Expr::Field(Self::Access {
                    expr,
                    s0,
                    s1,
                    index: Box::new(Expr::Lit(Lit::String(StringLit::from_ident(ident)))),
                    s2: Space::empty(span),
                    span,
                });
                (new, true)
            }

            Self::AssignIdent {
                expr,
                s0,
                s1,
                ident,
                s2,
                s3,
                value,
                span,
            } => {
                // `expr s0 . s1 ident s2 = s3 value`
                // -> `expr s0 [ s1 ident_str ] s2 = s3 value`
                let new = Expr::Field(Self::Assign {
                    expr,
                    s0,
                    s1,
                    index: Box::new(Expr::Lit(Lit::String(StringLit::from_ident(ident)))),
                    s2: Space::empty(span),
                    s3: s2,
                    s4: s3,
                    value,
                    span,
                });
                (new, true)
            }
        }
    }
}

use crate::ast::{
    Call, Expr, Field, Lit, Separated, Space, TableConstr, TableConstrElem, TableLitElem,
};
use crate::builtin::Builtin;

impl Field {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                expr,
                s0,
                s1,
                index,
                s2,
                span,
            } => {
                let (expr, desugared) = expr.desugar();
                if desugared {
                    let new = Expr::Field(Self::Access {
                        expr: Box::new(expr),
                        s0,
                        s1,
                        index,
                        s2,
                        span,
                    });
                    return (new, true);
                }

                let (index, desugared) = index.desugar();
                if desugared {
                    let new = Expr::Field(Self::Access {
                        expr: Box::new(expr),
                        s0,
                        s1,
                        index: Box::new(index),
                        s2,
                        span,
                    });
                    return (new, true);
                }

                let elems = Separated::NonEmpty {
                    first_elem: TableConstrElem::Lit(TableLitElem::Positional(Box::new(expr))),
                    last_elems: vec![(
                        (Space::empty(span), Space::empty(span)),
                        TableConstrElem::Lit(TableLitElem::Positional(Box::new(index))),
                    )],
                    trailing: None,
                    span,
                };
                let constr = TableConstr {
                    s0: Space::empty(span),
                    elems,
                    s1: Space::empty(span),
                    span,
                };
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
                s0,
                s1,
                index,
                s2,
                s3,
                s4,
                value,
                span,
            } => {
                let new = Expr::Field(Self::Assign {
                    expr,
                    s0,
                    s1,
                    index,
                    s2,
                    s3,
                    s4,
                    value,
                    span,
                });
                (new, true) // TODO Implement
            }

            Self::AccessIdent {
                expr,
                s0,
                s1,
                ident,
                span,
            } => {
                let new = Expr::Field(Self::AccessIdent {
                    expr,
                    s0,
                    s1,
                    ident,
                    span,
                });
                (new, false) // TODO Implement
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
                let new = Expr::Field(Self::AssignIdent {
                    expr,
                    s0,
                    s1,
                    ident,
                    s2,
                    s3,
                    value,
                    span,
                });
                (new, false) // TODO Implement
            }
        }
    }
}

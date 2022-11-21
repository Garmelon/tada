use crate::ast::{
    Call, Expr, Field, Line, Lit, Separated, Space, TableConstr, TableConstrElem, TableLitElem,
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
                // ` expr s0 [ s1 index s2 ]`
                // -> `'get s0 { expr, s1 index s2 }`
                let elems = Separated::NonEmpty {
                    first_elem: TableConstrElem::Lit(TableLitElem::Positional(expr)),
                    last_elems: vec![(
                        (Space::empty(span), s1),
                        TableConstrElem::Lit(TableLitElem::Positional(index)),
                    )],
                    trailing: None,
                    span,
                };
                let constr = TableConstr {
                    s0: Space::empty(span),
                    elems,
                    s1: s2,
                    span,
                };
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Get, span))),
                    s0,
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
                // `expr s0 [ s1 index s2 ] s3 = s4 value`
                // -> `'set s0 { expr, s1 index s2, s3 s4 value }`
                let elems = Separated::NonEmpty {
                    first_elem: TableConstrElem::Lit(TableLitElem::Positional(expr)),
                    last_elems: vec![
                        (
                            (Space::empty(span), s1),
                            TableConstrElem::Lit(TableLitElem::Positional(index)),
                        ),
                        (
                            (s2, s3.then_line(Line::Empty).then(s4)),
                            TableConstrElem::Lit(TableLitElem::Positional(value)),
                        ),
                    ],
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
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Set, span))),
                    s0,
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

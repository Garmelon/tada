use crate::ast::{
    Call, Expr, Field, Line, Lit, Separated, Space, StringLit, StringLitElem, TableConstr,
    TableConstrElem, TableLitElem,
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
                // `expr s0 . s1 ident´
                // -> `expr s0 [ s1 ident_str ]`
                let ident_str = Expr::Lit(Lit::String(StringLit {
                    elems: vec![StringLitElem::Plain(ident.name)],
                    span,
                }));
                let new = Expr::Field(Self::Access {
                    expr,
                    s0,
                    s1,
                    index: Box::new(ident_str),
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
                let ident_str = Expr::Lit(Lit::String(StringLit {
                    elems: vec![StringLitElem::Plain(ident.name)],
                    span,
                }));
                let new = Expr::Field(Self::Assign {
                    expr,
                    s0,
                    s1,
                    index: Box::new(ident_str),
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

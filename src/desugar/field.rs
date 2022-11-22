use crate::ast::{
    BoundedSeparated, Call, Expr, Field, Line, Lit, Space, StringLit, TableConstr, TableConstrElem,
    TableLitElem,
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
                let elems = vec![
                    (
                        Space::empty(span),
                        TableConstrElem::Lit(TableLitElem::Positional(expr)),
                        Space::empty(span),
                    ),
                    (
                        s1,
                        TableConstrElem::Lit(TableLitElem::Positional(index)),
                        s2,
                    ),
                ];
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Get, span))),
                    s0,
                    constr: TableConstr(BoundedSeparated {
                        elems,
                        trailing: None,
                        span,
                    }),
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
                let elems = vec![
                    (
                        Space::empty(span),
                        TableConstrElem::Lit(TableLitElem::Positional(expr)),
                        Space::empty(span),
                    ),
                    (
                        s1,
                        TableConstrElem::Lit(TableLitElem::Positional(index)),
                        s2,
                    ),
                    (
                        s3.then_line(Line::Empty).then(s4),
                        TableConstrElem::Lit(TableLitElem::Positional(value)),
                        Space::empty(span),
                    ),
                ];
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Set, span))),
                    s0,
                    constr: TableConstr(BoundedSeparated {
                        elems,
                        trailing: None,
                        span,
                    }),
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

use crate::ast::{
    BoundedSeparated, Call, Expr, Field, Line, Lit, Space, StringLit, TableConstr, TableConstrElem,
    TableLitElem, Var,
};
use crate::builtin::Builtin;
use crate::span::HasSpan;

impl Var {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Access {
                s0,
                index,
                s1,
                span,
            } => {
                // `[ s0 index s1 ]`
                // -> `'scope()[ s0 index s1 ]`
                let scope = Expr::Call(Call::NoArg {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Scope, span))),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                });
                let new = Expr::Field(Field::Access {
                    expr: Box::new(scope),
                    s0: Space::empty(span),
                    s1: s0,
                    index,
                    s2: s1,
                    span,
                });
                (new, true)
            }

            Self::Assign {
                local: None,
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span,
            } => {
                // `[ s0 index s1 ] s2 = s3 value`
                // -> `'scope()[ s0 index s1 ] s2 = s3 value`
                let scope = Expr::Call(Call::NoArg {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Scope, span))),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                });
                let new = Expr::Field(Field::Assign {
                    expr: Box::new(scope),
                    s0: Space::empty(span),
                    s1: s0,
                    index,
                    s2: s1,
                    s3: s2,
                    s4: s3,
                    value,
                    span,
                });
                (new, true)
            }

            Self::Assign {
                local: Some(local),
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span,
            } => {
                // `local [ s0 index s1 ] s2 = s3 value`
                // --> `'setraw { 'scope(), local s0 index s1, s2 s3 value }`
                let scope = Expr::Call(Call::NoArg {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::Scope, span))),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    span,
                });
                let elems = vec![
                    (
                        Space::empty(span),
                        TableConstrElem::Lit(TableLitElem::Positional(Box::new(scope))),
                        Space::empty(span),
                    ),
                    (
                        local.then_line(Line::Empty).then(s0),
                        TableConstrElem::Lit(TableLitElem::Positional(index)),
                        s1,
                    ),
                    (
                        s2.then_line(Line::Empty).then(s3),
                        TableConstrElem::Lit(TableLitElem::Positional(value)),
                        Space::empty(span),
                    ),
                ];
                let new = Expr::Call(Call::Constr {
                    expr: Box::new(Expr::Lit(Lit::Builtin(Builtin::SetRaw, span))),
                    s0: Space::empty(span),
                    constr: TableConstr(BoundedSeparated {
                        elems,
                        trailing: None,
                        span,
                    }),
                    span,
                });
                (new, true)
            }

            Self::AccessIdent(name) => {
                // `name`
                // -> `[ name_str ]`
                let span = name.span();
                let new = Expr::Var(Self::Access {
                    s0: Space::empty(span),
                    index: Box::new(Expr::Lit(Lit::String(StringLit::from_ident(name)))),
                    s1: Space::empty(span),
                    span,
                });
                (new, true)
            }

            Self::AssignIdent {
                local,
                name,
                s0,
                s1,
                value,
                span,
            } => {
                // `local name s0 = s1 value`
                // -> `local [ name_str ] s0 = s1 value`
                let new = Expr::Var(Self::Assign {
                    local,
                    s0: Space::empty(span),
                    index: Box::new(Expr::Lit(Lit::String(StringLit::from_ident(name)))),
                    s1: Space::empty(span),
                    s2: s0,
                    s3: s1,
                    value,
                    span,
                });
                (new, true)
            }
        }
    }
}

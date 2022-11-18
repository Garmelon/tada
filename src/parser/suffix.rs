//! Corresponds to `ast::call` and `ast::field`.

use chumsky::prelude::*;

use crate::ast::{Call, Expr, Field, Ident, Space, TableConstr};
use crate::span::{HasSpan, Span};

use super::basic::{space, Error};

enum Suffix {
    CallArg {
        s0: Space,
        s1: Space,
        arg: Box<Expr>,
        s2: Space,
    },

    CallNoArg {
        s0: Space,
        s1: Space,
    },

    CallConstr {
        s0: Space,
        constr: TableConstr,
    },

    FieldAccess {
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
    },

    FieldAssign {
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        s3: Space,
        s4: Space,
        value: Box<Expr>,
    },

    FieldAccessIdent {
        s0: Space,
        s1: Space,
        ident: Ident,
    },

    FieldAssignIdent {
        s0: Space,
        s1: Space,
        ident: Ident,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
    },
}

impl Suffix {
    fn into_expr(self, span: Span, expr: Expr) -> Expr {
        let expr = Box::new(expr);
        match self {
            Suffix::CallArg { s0, s1, arg, s2 } => Expr::Call(Call::Arg {
                expr,
                s0,
                s1,
                arg,
                s2,
                span,
            }),
            Suffix::CallNoArg { s0, s1 } => Expr::Call(Call::NoArg { expr, s0, s1, span }),
            Suffix::CallConstr { s0, constr } => Expr::Call(Call::Constr {
                expr,
                s0,
                constr,
                span,
            }),
            Suffix::FieldAccess { s0, s1, index, s2 } => Expr::Field(Field::Access {
                expr,
                s0,
                s1,
                index,
                s2,
                span,
            }),
            Suffix::FieldAssign {
                s0,
                s1,
                index,
                s2,
                s3,
                s4,
                value,
            } => Expr::Field(Field::Assign {
                expr,
                s0,
                s1,
                index,
                s2,
                s3,
                s4,
                value,
                span,
            }),
            Suffix::FieldAccessIdent { s0, s1, ident } => Expr::Field(Field::AccessIdent {
                expr,
                s0,
                s1,
                ident,
                span,
            }),
            Suffix::FieldAssignIdent {
                s0,
                s1,
                ident,
                s2,
                s3,
                value,
            } => Expr::Field(Field::AssignIdent {
                expr,
                s0,
                s1,
                ident,
                s2,
                s3,
                value,
                span,
            }),
        }
    }
}

fn suffix_field_access(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> {
    space()
        .then_ignore(just('['))
        .then(space())
        .then(expr)
        .then(space())
        .then_ignore(just(']'))
        .map(|(((s0, s1), index), s2)| Suffix::FieldAccess {
            s0,
            s1,
            index: Box::new(index),
            s2,
        })
}

pub fn suffixed(
    atom: impl Parser<char, Expr, Error = Error>,
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    let field_access = suffix_field_access(expr.clone());

    let suffix = field_access.map_with_span(|suffix, span| (suffix, span));

    atom.then(suffix.repeated())
        .foldl(|expr, (suffix, span)| suffix.into_expr(expr.span().join(span), expr))
}

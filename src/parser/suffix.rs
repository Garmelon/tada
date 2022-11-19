//! Corresponds to `ast::call` and `ast::field`.

use chumsky::prelude::*;

use crate::ast::{Call, Expr, Field, Ident, Space, TableConstr};
use crate::span::{HasSpan, Span};

use super::basic::{ident, space, Error};
use super::table_constr::table_constr;

enum Suffix {
    /// See [`Call::Arg`].
    CallArg {
        s0: Space,
        s1: Space,
        arg: Box<Expr>,
        s2: Space,
    },

    /// See [`Call::NoArg`].
    CallNoArg { s0: Space, s1: Space },

    /// See [`Call::Constr`].
    CallConstr { s0: Space, constr: TableConstr },

    /// See [`Field::Access`].
    FieldAccess {
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
    },

    /// See [`Field::Assign`].
    FieldAssign {
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        s3: Space,
        s4: Space,
        value: Box<Expr>,
    },

    /// See [`Field::AccessIdent`].
    FieldAccessIdent { s0: Space, s1: Space, ident: Ident },

    /// See [`Field::AssignIdent`].
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

fn suffix_call_arg(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then_ignore(just('('))
        .then(space())
        .then(expr)
        .then(space())
        .then_ignore(just(')'))
        .map(|(((s0, s1), arg), s2)| Suffix::CallArg {
            s0,
            s1,
            arg: Box::new(arg),
            s2,
        })
}

fn suffix_call_no_arg() -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then_ignore(just('('))
        .then(space())
        .then_ignore(just(')'))
        .map(|(s0, s1)| Suffix::CallNoArg { s0, s1 })
}

fn suffix_call_constr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then(table_constr(expr))
        .map(|(s0, constr)| Suffix::CallConstr { s0, constr })
}

fn suffix_field_access(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> + Clone {
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

fn suffix_field_assign(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then_ignore(just('['))
        .then(space())
        .then(expr.clone())
        .then(space())
        .then_ignore(just(']'))
        .then(space())
        .then_ignore(just('='))
        .then(space())
        .then(expr)
        .map(
            |((((((s0, s1), index), s2), s3), s4), value)| Suffix::FieldAssign {
                s0,
                s1,
                index: Box::new(index),
                s2,
                s3,
                s4,
                value: Box::new(value),
            },
        )
}

fn suffix_field_access_ident() -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then_ignore(just('.'))
        .then(space())
        .then(ident())
        .map(|((s0, s1), ident)| Suffix::FieldAccessIdent { s0, s1, ident })
}

fn suffix_field_assign_ident(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Suffix, Error = Error> + Clone {
    space()
        .then_ignore(just('.'))
        .then(space())
        .then(ident())
        .then(space())
        .then_ignore(just('='))
        .then(space())
        .then(expr)
        .map(
            |(((((s0, s1), ident), s2), s3), value)| Suffix::FieldAssignIdent {
                s0,
                s1,
                ident,
                s2,
                s3,
                value: Box::new(value),
            },
        )
}

pub fn suffixed(
    atom: impl Parser<char, Expr, Error = Error> + Clone,
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> + Clone {
    let call_arg = suffix_call_arg(expr.clone());
    let call_no_arg = suffix_call_no_arg();
    let call_constr = suffix_call_constr(expr.clone());
    let field_access = suffix_field_access(expr.clone());
    let field_assign = suffix_field_assign(expr.clone());
    let field_access_ident = suffix_field_access_ident();
    let field_assign_ident = suffix_field_assign_ident(expr);

    let suffix = call_arg
        .or(call_no_arg)
        .or(call_constr)
        .or(field_assign)
        .or(field_access)
        .or(field_assign_ident)
        .or(field_access_ident)
        .map_with_span(|suffix, span| (suffix, span));

    atom.then(suffix.repeated())
        .foldl(|expr, (suffix, span)| suffix.into_expr(expr.span().join(span), expr))
}

//! Corresponds to `ast::var`.

use chumsky::prelude::*;

use crate::ast::{Expr, Var};

use super::basic::{ident, space, Error};

fn var_access(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Var, Error = Error> {
    just("[")
        .ignore_then(space())
        .then(expr)
        .then(space())
        .then_ignore(just("]"))
        .map_with_span(|((s0, index), s1), span| Var::Access {
            s0,
            index: Box::new(index),
            s1,
            span,
        })
}

fn var_assign(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Var, Error = Error> {
    let local = text::keyword("local").ignore_then(space()).or_not();

    local
        .then_ignore(just("["))
        .then(space())
        .then(expr.clone())
        .then(space())
        .then_ignore(just("]"))
        .then(space())
        .then_ignore(just("="))
        .then(space())
        .then(expr)
        .map_with_span(
            |((((((local, s0), index), s1), s2), s3), value), span| Var::Assign {
                local,
                s0,
                index: Box::new(index),
                s1,
                s2,
                s3,
                value: Box::new(value),
                span,
            },
        )
}

fn var_assign_ident(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Var, Error = Error> {
    let local = text::keyword("local").ignore_then(space()).or_not();

    local
        .then(ident())
        .then(space())
        .then_ignore(just("="))
        .then(space())
        .then(expr)
        .map_with_span(
            |((((local, name), s0), s1), value), span| Var::AssignIdent {
                local,
                name,
                s0,
                s1,
                value: Box::new(value),
                span,
            },
        )
}

pub fn var(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Var, Error = Error> {
    let access = var_access(expr.clone());
    let assign = var_assign(expr.clone());
    let access_ident = ident().map(Var::AccessIdent);
    let assign_ident = var_assign_ident(expr);

    assign.or(access).or(assign_ident).or(access_ident)
}

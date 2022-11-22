//! Corresponds to `ast::var`.

use chumsky::prelude::*;

use crate::ast::{Expr, Ident, Space, Var};

use super::basic::{EParser, Error};

fn var_access(space: EParser<Space>, expr: EParser<Expr>) -> impl Parser<char, Var, Error = Error> {
    just('[')
        .ignore_then(space.clone())
        .then(expr)
        .then(space)
        .then_ignore(just(']'))
        .map_with_span(|((s0, index), s1), span| Var::Access {
            s0,
            index: index.boxed(),
            s1,
            span,
        })
}

fn var_assign(
    space: EParser<Space>,
    local: EParser<Option<Space>>,
    expr: EParser<Expr>,
) -> impl Parser<char, Var, Error = Error> {
    local
        .then_ignore(just('['))
        .then(space.clone())
        .then(expr.clone())
        .then(space.clone())
        .then_ignore(just(']'))
        .then(space.clone())
        .then_ignore(just('='))
        .then(space)
        .then(expr)
        .map_with_span(
            |((((((local, s0), index), s1), s2), s3), value), span| Var::Assign {
                local,
                s0,
                index: index.boxed(),
                s1,
                s2,
                s3,
                value: value.boxed(),
                span,
            },
        )
}

fn var_assign_ident(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    expr: EParser<Expr>,
) -> impl Parser<char, Var, Error = Error> {
    local
        .then(ident)
        .then(space.clone())
        .then_ignore(just('='))
        .then(space)
        .then(expr)
        .map_with_span(
            |((((local, name), s0), s1), value), span| Var::AssignIdent {
                local,
                name,
                s0,
                s1,
                value: value.boxed(),
                span,
            },
        )
}

pub fn var(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    expr: EParser<Expr>,
) -> EParser<Var> {
    let access = var_access(space.clone(), expr.clone());
    let assign = var_assign(space.clone(), local.clone(), expr.clone());
    let access_ident = ident.clone().map(Var::AccessIdent);
    let assign_ident = var_assign_ident(space, ident, local, expr);

    assign.or(access).or(assign_ident).or(access_ident).boxed()
}

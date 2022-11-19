use chumsky::prelude::*;

use crate::ast::{Expr, FuncDef};

use super::basic::{ident, space, Error};
use super::table_destr::table_pattern;

fn func_def_anon_no_arg(
    expr: impl Parser<char, Expr, Error = Error>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space())
        .then_ignore(just('('))
        .then(space())
        .then_ignore(just(')'))
        .then(space())
        .then(expr)
        .map_with_span(|(((s0, s1), s2), body), span| FuncDef::AnonNoArg {
            s0,
            s1,
            s2,
            body: Box::new(body),
            span,
        })
}

fn func_def_anon_arg(
    expr: impl Parser<char, Expr, Error = Error>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space())
        .then_ignore(just('('))
        .then(space())
        .then(ident())
        .then(space())
        .then_ignore(just(')'))
        .then(space())
        .then(expr)
        .map_with_span(
            |(((((s0, s1), arg), s2), s3), body), span| FuncDef::AnonArg {
                s0,
                s1,
                arg,
                s2,
                s3,
                body: Box::new(body),
                span,
            },
        )
}

fn func_def_anon_destr(
    expr: impl Parser<char, Expr, Error = Error>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space())
        .then(table_pattern())
        .then(space())
        .then(expr)
        .map_with_span(|(((s0, pattern), s1), body), span| FuncDef::AnonDestr {
            s0,
            pattern,
            s1,
            body: Box::new(body),
            span,
        })
}

pub fn func_def(
    expr: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> BoxedParser<'static, char, FuncDef, Error> {
    func_def_anon_no_arg(expr.clone())
        .or(func_def_anon_arg(expr.clone()))
        .or(func_def_anon_destr(expr))
        .boxed()
}

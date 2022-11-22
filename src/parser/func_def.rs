use chumsky::prelude::*;

use crate::ast::{Expr, FuncDef, Ident, Space, TablePattern};

use super::basic::{EParser, Error};

fn func_def_anon_no_arg(
    space: EParser<Space>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space.clone())
        .then_ignore(just('('))
        .then(space.clone())
        .then_ignore(just(')'))
        .then(space)
        .then(expr)
        .map_with_span(|(((s0, s1), s2), body), span| FuncDef::AnonNoArg {
            s0,
            s1,
            s2,
            body: body.boxed(),
            span,
        })
}

fn func_def_anon_arg(
    space: EParser<Space>,
    ident: EParser<Ident>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space.clone())
        .then_ignore(just('('))
        .then(space.clone())
        .then(ident)
        .then(space.clone())
        .then_ignore(just(')'))
        .then(space)
        .then(expr)
        .map_with_span(
            |(((((s0, s1), arg), s2), s3), body), span| FuncDef::AnonArg {
                s0,
                s1,
                arg,
                s2,
                s3,
                body: body.boxed(),
                span,
            },
        )
}

fn func_def_anon_destr(
    space: EParser<Space>,
    table_pattern: EParser<TablePattern>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("function")
        .ignore_then(space.clone())
        .then(table_pattern)
        .then(space)
        .then(expr)
        .map_with_span(|(((s0, pattern), s1), body), span| FuncDef::AnonDestr {
            s0,
            pattern,
            s1,
            body: body.boxed(),
            span,
        })
}

fn func_def_named_no_arg(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    local
        .then_ignore(text::keyword("function"))
        .then(space.clone())
        .then(ident)
        .then(space.clone())
        .then_ignore(just('('))
        .then(space.clone())
        .then_ignore(just(')'))
        .then(space)
        .then(expr)
        .map_with_span(
            |((((((local, s0), name), s1), s2), s3), body), span| FuncDef::NamedNoArg {
                local,
                s0,
                name,
                s1,
                s2,
                s3,
                body: body.boxed(),
                span,
            },
        )
}

fn func_def_named_arg(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    local
        .then_ignore(text::keyword("function"))
        .then(space.clone())
        .then(ident.clone())
        .then(space.clone())
        .then_ignore(just('('))
        .then(space.clone())
        .then(ident)
        .then(space.clone())
        .then_ignore(just(')'))
        .then(space)
        .then(expr)
        .map_with_span(
            |((((((((local, s0), name), s1), s2), arg), s3), s4), body), span| FuncDef::NamedArg {
                local,
                s0,
                name,
                s1,
                s2,
                arg,
                s3,
                s4,
                body: body.boxed(),
                span,
            },
        )
}

fn func_def_named_destr(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    table_pattern: EParser<TablePattern>,
    expr: EParser<Expr>,
) -> impl Parser<char, FuncDef, Error = Error> {
    local
        .then_ignore(text::keyword("function"))
        .then(space.clone())
        .then(ident)
        .then(space.clone())
        .then(table_pattern)
        .then(space)
        .then(expr)
        .map_with_span(|((((((local, s0), name), s1), pattern), s2), body), span| {
            FuncDef::NamedDestr {
                local,
                s0,
                name,
                s1,
                pattern,
                s2,
                body: body.boxed(),
                span,
            }
        })
}

pub fn func_def(
    space: EParser<Space>,
    ident: EParser<Ident>,
    local: EParser<Option<Space>>,
    table_pattern: EParser<TablePattern>,
    expr: EParser<Expr>,
) -> EParser<FuncDef> {
    let anon_no_arg = func_def_anon_no_arg(space.clone(), expr.clone());
    let anon_arg = func_def_anon_arg(space.clone(), ident.clone(), expr.clone());
    let anon_destr =
        func_def_anon_destr(space.clone(), table_pattern.clone(), expr.clone().clone());
    let named_no_arg = func_def_named_no_arg(
        space.clone(),
        ident.clone(),
        local.clone(),
        expr.clone().clone(),
    );
    let named_arg = func_def_named_arg(
        space.clone(),
        ident.clone(),
        local.clone(),
        expr.clone().clone(),
    );
    let named_destr = func_def_named_destr(space, ident, local, table_pattern, expr);

    anon_no_arg
        .or(anon_arg)
        .or(anon_destr)
        .or(named_no_arg)
        .or(named_arg)
        .or(named_destr)
        .boxed()
}

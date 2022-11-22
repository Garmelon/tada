//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, Ident, Space, TableDestr, TablePattern, TablePatternElem};

use super::basic::{bounded_separated, EParser, Error};

fn table_pattern_elem(
    space: EParser<Space>,
    ident: EParser<Ident>,
) -> impl Parser<char, TablePatternElem, Error = Error> + Clone {
    let positional = ident.clone().map(TablePatternElem::Positional);

    let named = ident
        .clone()
        .then(space.clone())
        .then_ignore(just(':'))
        .then(space)
        .then(ident)
        .map_with_span(|(((name, s0), s1), ident), span| TablePatternElem::Named {
            name,
            s0,
            s1,
            ident,
            span,
        });

    named.or(positional)
}

pub fn table_pattern(space: EParser<Space>, ident: EParser<Ident>) -> EParser<TablePattern> {
    let elem = table_pattern_elem(space.clone(), ident);
    bounded_separated(
        space,
        just('{').to(()),
        just('}').to(()),
        just(',').to(()),
        elem,
    )
    .map(TablePattern)
    .boxed()
}

pub fn table_destr(
    space: EParser<Space>,
    local: EParser<Option<Space>>,
    table_pattern: EParser<TablePattern>,
    expr: EParser<Expr>,
) -> EParser<TableDestr> {
    local
        .then(table_pattern)
        .then(space.clone())
        .then_ignore(just('='))
        .then(space)
        .then(expr)
        .map_with_span(|((((local, pattern), s0), s1), value), span| TableDestr {
            local,
            pattern,
            s0,
            s1,
            value: value.boxed(),
            span,
        })
        .boxed()
}

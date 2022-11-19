//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, Ident, Space, TableDestr, TablePattern, TablePatternElem};

use super::basic::{EParser, Error};

fn table_pattern_elem(
    space: EParser<Space>,
    ident: EParser<Ident>,
) -> impl Parser<char, TablePatternElem, Error = Error> {
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
    let elem = space
        .clone()
        .then(table_pattern_elem(space.clone(), ident))
        .then(space.clone())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space).or_not();

    let elems = elem.separated_by(just(',')).then(trailing_comma);

    just('{')
        .ignore_then(elems)
        .then_ignore(just('}'))
        .map_with_span(|(elems, trailing_comma), span| TablePattern {
            elems,
            trailing_comma,
            span,
        })
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
            value: Box::new(value),
            span,
        })
        .boxed()
}

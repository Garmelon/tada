//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, TableDestr, TablePattern, TablePatternElem};

use super::basic::{ident, space, Error};

pub fn table_pattern_elem() -> impl Parser<char, TablePatternElem, Error = Error> + Clone {
    let positional = ident().map(TablePatternElem::Positional);

    let named = ident()
        .then(space())
        .then_ignore(just(':'))
        .then(space())
        .then(ident())
        .map_with_span(|(((name, s0), s1), ident), span| TablePatternElem::Named {
            name,
            s0,
            s1,
            ident,
            span,
        });

    named.or(positional)
}

pub fn table_pattern() -> impl Parser<char, TablePattern, Error = Error> + Clone {
    let elem = space()
        .then(table_pattern_elem())
        .then(space())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space()).or_not();

    let elems = elem.separated_by(just(',')).then(trailing_comma);

    just('{')
        .ignore_then(elems)
        .then_ignore(just('}'))
        .map_with_span(|(elems, trailing_comma), span| TablePattern {
            elems,
            trailing_comma,
            span,
        })
}

pub fn table_destr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableDestr, Error = Error> + Clone {
    let local = text::keyword("local").ignore_then(space()).or_not();

    local
        .then(table_pattern())
        .then(space())
        .then_ignore(just('='))
        .then(space())
        .then(expr)
        .map_with_span(|((((local, pattern), s0), s1), value), span| TableDestr {
            local,
            pattern,
            s0,
            s1,
            value: Box::new(value),
            span,
        })
}

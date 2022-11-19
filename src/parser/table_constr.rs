//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, TableConstr, TableConstrElem};

use super::basic::{space, Error};
use super::lit::table_lit_elem;

pub fn table_constr_elem(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableConstrElem, Error = Error> + Clone {
    let lit = table_lit_elem(expr.clone()).map(TableConstrElem::Lit);

    let indexed = just('[')
        .ignore_then(space())
        .then(expr.clone())
        .then(space())
        .then_ignore(just(']'))
        .then(space())
        .then_ignore(just(':'))
        .then(space())
        .then(expr)
        .map_with_span(
            |(((((s0, index), s1), s2), s3), value), span| TableConstrElem::Indexed {
                s0,
                index: Box::new(index),
                s1,
                s2,
                s3,
                value: Box::new(value),
                span,
            },
        );

    indexed.or(lit)
}

pub fn table_constr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableConstr, Error = Error> + Clone {
    let elem = space()
        .then(table_constr_elem(expr))
        .then(space())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space()).or_not();

    let elems = elem.separated_by(just(',')).then(trailing_comma);

    just('{')
        .ignore_then(elems)
        .then_ignore(just('}'))
        .map_with_span(|(elems, trailing_comma), span| TableConstr {
            elems,
            trailing_comma,
            span,
        })
}

//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, Space, TableConstr, TableConstrElem, TableLitElem};

use super::basic::{EParser, Error};

fn table_constr_elem(
    space: EParser<Space>,
    table_lit_elem: EParser<TableLitElem>,
    expr: EParser<Expr>,
) -> impl Parser<char, TableConstrElem, Error = Error> {
    let lit = table_lit_elem.map(TableConstrElem::Lit);

    let indexed = just('[')
        .ignore_then(space.clone())
        .then(expr.clone())
        .then(space.clone())
        .then_ignore(just(']'))
        .then(space.clone())
        .then_ignore(just(':'))
        .then(space)
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
    space: EParser<Space>,
    table_lit_elem: EParser<TableLitElem>,
    expr: EParser<Expr>,
) -> EParser<TableConstr> {
    let elem = space
        .clone()
        .then(table_constr_elem(space.clone(), table_lit_elem, expr))
        .then(space.clone())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space).or_not();

    let elems = elem.separated_by(just(',')).then(trailing_comma);

    just('{')
        .ignore_then(elems)
        .then_ignore(just('}'))
        .map_with_span(|(elems, trailing_comma), span| TableConstr {
            elems,
            trailing_comma,
            span,
        })
        .boxed()
}

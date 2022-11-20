//! Corresponds to `ast::table_constr`.

use chumsky::prelude::*;

use crate::ast::{Expr, Space, TableConstr, TableConstrElem, TableLitElem};

use super::basic::{separated_by, EParser, Error};

fn table_constr_elem(
    space: EParser<Space>,
    table_lit_elem: EParser<TableLitElem>,
    expr: EParser<Expr>,
) -> impl Parser<char, TableConstrElem, Error = Error> + Clone {
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
    let elem = table_constr_elem(space.clone(), table_lit_elem, expr);
    let separator = space.clone().then_ignore(just(',')).then(space.clone());
    let trailing_separator = space.clone().then_ignore(just(','));

    space
        .clone()
        .then(separated_by(elem, separator, trailing_separator))
        .then(space)
        .delimited_by(just('{'), just('}'))
        .map_with_span(|((s0, elems), s1), span| TableConstr {
            s0,
            elems,
            s1,
            span,
        })
        .boxed()
}

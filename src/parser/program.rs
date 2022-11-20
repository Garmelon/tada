//! Corresponds to `ast::program`.

use chumsky::prelude::*;

use crate::ast::{Expr, Program, Space, TableLitElem};

use super::basic::{separated_by, EParser};

pub fn program(
    space: EParser<Space>,
    table_lit_elem: EParser<TableLitElem>,
    expr: EParser<Expr>,
) -> EParser<Program> {
    let lit = space
        .clone()
        .then(expr)
        .then(space.clone())
        .map_with_span(|((s0, expr), s1), span| Program::Expr { s0, expr, s1, span });

    let separator = space.clone().then_ignore(just(',')).then(space.clone());
    let trailing_separator = space.clone().then_ignore(just(','));
    let module = space
        .clone()
        .then_ignore(text::keyword("module"))
        .then(space.clone())
        .then(separated_by(table_lit_elem, separator, trailing_separator))
        .then(space.clone())
        .map_with_span(|(((s0, s1), elems), s2), span| Program::Module {
            s0,
            s1,
            elems,
            s2,
            span,
        });

    module.or(lit).boxed()
}

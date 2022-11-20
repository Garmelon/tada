//! Corresponds to `ast::program`.

use chumsky::prelude::*;

use crate::ast::{Expr, Program, Space, TableLitElem};

use super::basic::EParser;

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

    let elem = space
        .clone()
        .then(table_lit_elem)
        .then(space.clone())
        .map(|((s0, elem), s1)| (s0, elem, s1));
    let trailing_comma = just(',').ignore_then(space.clone()).or_not();
    let module = space
        .then_ignore(text::keyword("module"))
        .then(elem.separated_by(just(',')))
        .then(trailing_comma)
        .map_with_span(|((s0, elems), trailing_comma), span| Program::Module {
            s0,
            elems,
            trailing_comma,
            span,
        });

    module.or(lit).boxed()
}

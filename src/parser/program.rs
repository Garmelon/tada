//! Corresponds to `ast::program`.

use chumsky::prelude::*;

use crate::ast::{Expr, Program, Space, TableLitElem};

use super::basic::{bounded_separated, EParser};

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

    let module = space
        .clone()
        .then_ignore(text::keyword("module"))
        .then(bounded_separated(
            space,
            empty(),
            empty(),
            just(',').to(()),
            table_lit_elem,
        ))
        .map_with_span(|(s0, elems), span| Program::Module { s0, elems, span });

    module.or(lit).boxed()
}

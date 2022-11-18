//! Corresponds to `ast::expr`.

use chumsky::prelude::*;

use crate::ast::Expr;

use super::basic::{space, Error};
use super::lit::lit;
use super::suffix::suffixed;
use super::table_constr::table_constr;
use super::table_destr::table_destr;
use super::var::var;

fn atom_paren(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    just("(")
        .ignore_then(space())
        .then(expr)
        .then(space())
        .then_ignore(just(")"))
        .map_with_span(|((s0, inner), s1), span| Expr::Paren {
            s0,
            inner: Box::new(inner),
            s1,
            span,
        })
}

fn atom(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    let lit = lit(expr.clone()).map(Expr::Lit);
    let var = var(expr.clone()).map(Expr::Var);
    let table_constr = table_constr(expr.clone()).map(Expr::TableConstr);
    let table_destr = table_destr(expr.clone()).map(Expr::TableDestr);
    let paren = atom_paren(expr);

    lit.or(paren).or(table_destr).or(table_constr).or(var)
}

pub fn expr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    suffixed(atom(expr.clone()), expr)
}

//! Corresponds to `ast::expr`.

use chumsky::prelude::*;

use crate::ast::{BinOp, Expr};
use crate::span::HasSpan;

use super::basic::{space, Error};
use super::lit::lit;
use super::prefix::prefixed;
use super::suffix::suffixed;
use super::table_constr::table_constr;
use super::table_destr::table_destr;
use super::var::var;

fn atom_paren(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> + Clone {
    just('(')
        .ignore_then(space())
        .then(expr)
        .then(space())
        .then_ignore(just(')'))
        .map_with_span(|((s0, inner), s1), span| Expr::Paren {
            s0,
            inner: Box::new(inner),
            s1,
            span,
        })
}

fn atom(
    expr: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> impl Parser<char, Expr, Error = Error> + Clone {
    let lit = lit(expr.clone()).map(Expr::Lit);
    let var = var(expr.clone()).map(Expr::Var);
    let table_constr = table_constr(expr.clone()).map(Expr::TableConstr);
    let table_destr = table_destr(expr.clone()).map(Expr::TableDestr);
    let paren = atom_paren(expr.clone());

    let base = lit.or(paren).or(table_destr).or(table_constr).or(var);
    prefixed(suffixed(base, expr))
}

fn left_assoc(
    op: impl Parser<char, BinOp, Error = Error> + Clone + 'static,
    over: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> BoxedParser<'static, char, Expr, Error> {
    let op_over = space()
        .then(op)
        .then(space())
        .then(over.clone())
        .map(|(((s0, op), s1), right)| (s0, op, s1, right));

    over.then(op_over.repeated())
        .foldl(|left, (s0, op, s1, right)| Expr::BinOp {
            span: left.span().join(right.span()),
            left: Box::new(left),
            s0,
            op,
            s1,
            right: Box::new(right),
        })
        .boxed()
}

pub fn expr(
    expr: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> BoxedParser<'static, char, Expr, Error> {
    // * / %
    let prec0 = (just('*').to(BinOp::Mul))
        .or(just('/').to(BinOp::Div))
        .or(just('%').to(BinOp::Mod));

    // + -
    let prec1 = (just('+').to(BinOp::Add)).or(just('-').to(BinOp::Sub));

    // == != > >= < <=
    let prec2 = (just("==").to(BinOp::Eq))
        .or(just("!=").to(BinOp::Neq))
        .or(just('>').to(BinOp::Gt))
        .or(just(">=").to(BinOp::Ge))
        .or(just('<').to(BinOp::Lt))
        .or(just("<=").to(BinOp::Le));

    // and
    let prec3 = text::keyword("and").to(BinOp::And);

    // or
    let prec4 = text::keyword("or").to(BinOp::Or);

    left_assoc(
        prec4,
        left_assoc(
            prec3,
            left_assoc(prec2, left_assoc(prec1, left_assoc(prec0, atom(expr)))),
        ),
    )
}

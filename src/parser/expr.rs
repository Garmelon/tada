//! Corresponds to `ast::expr`.

use chumsky::prelude::*;

use crate::ast::{BinOp, Expr};
use crate::span::HasSpan;

use super::basic::{space, Error};
use super::func_defs::func_def;
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
    let func_def = func_def(expr.clone()).map(Expr::FuncDef);
    let paren = atom_paren(expr.clone());

    let base = lit
        .or(paren)
        .or(table_destr)
        .or(table_constr)
        .or(func_def)
        .or(var);

    prefixed(suffixed(base, expr))
}

fn left_assoc(
    op: impl Parser<char, BinOp, Error = Error> + 'static,
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

fn right_assoc(
    op: impl Parser<char, BinOp, Error = Error> + 'static,
    over: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> BoxedParser<'static, char, Expr, Error> {
    let over_op = over
        .clone()
        .then(space())
        .then(op)
        .then(space())
        .map(|(((left, s0), op), s1)| (left, s0, op, s1));

    over_op
        .repeated()
        .then(over)
        .foldr(|(left, s0, op, s1), right| Expr::BinOp {
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
    let op_prec_4 = (just('*').to(BinOp::Mul))
        .or(just('/').to(BinOp::Div))
        .or(just('%').to(BinOp::Mod));

    // + -
    let op_prec_3 = (just('+').to(BinOp::Add)).or(just('-').to(BinOp::Sub));

    // == != > >= < <=
    let op_prec_2 = (just("==").to(BinOp::Eq))
        .or(just("!=").to(BinOp::Neq))
        .or(just('>').to(BinOp::Gt))
        .or(just(">=").to(BinOp::Ge))
        .or(just('<').to(BinOp::Lt))
        .or(just("<=").to(BinOp::Le));

    // and
    let op_prec_1 = text::keyword("and").to(BinOp::And);

    // or
    let op_prec_0 = text::keyword("or").to(BinOp::Or);

    right_assoc(
        op_prec_0,
        right_assoc(
            op_prec_1,
            left_assoc(
                op_prec_2,
                left_assoc(op_prec_3, left_assoc(op_prec_4, atom(expr))),
            ),
        ),
    )
}

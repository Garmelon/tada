//! Corresponds to `ast::expr`.

use chumsky::prelude::*;

use crate::ast::{BinOp, Expr, FuncDef, Lit, Space, TableConstr, TableDestr, Var};
use crate::span::HasSpan;

use super::basic::{EParser, Error};

fn atom_paren(
    space: EParser<Space>,
    expr: EParser<Expr>,
) -> impl Parser<char, Expr, Error = Error> + Clone {
    just('(')
        .ignore_then(space.clone())
        .then(expr)
        .then(space)
        .then_ignore(just(')'))
        .map_with_span(|((s0, inner), s1), span| Expr::Paren {
            s0,
            inner: Box::new(inner),
            s1,
            span,
        })
}

pub fn atom(
    space: EParser<Space>,
    lit: EParser<Lit>,
    var: EParser<Var>,
    table_constr: EParser<TableConstr>,
    table_destr: EParser<TableDestr>,
    func_def: EParser<FuncDef>,
    expr: EParser<Expr>,
) -> EParser<Expr> {
    let lit = lit.map(Expr::Lit);
    let var = var.map(Expr::Var);
    let table_constr = table_constr.map(Expr::TableConstr);
    let table_destr = table_destr.map(Expr::TableDestr);
    let func_def = func_def.map(Expr::FuncDef);
    let paren = atom_paren(space, expr);

    lit.or(paren)
        .or(table_destr)
        .or(table_constr)
        .or(func_def)
        .or(var)
        .boxed()
}

fn left_assoc(
    space: EParser<Space>,
    op: impl Parser<char, BinOp, Error = Error> + 'static,
    over: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> EParser<Expr> {
    let op_over = space
        .clone()
        .then(op)
        .then(space)
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
    space: EParser<Space>,
    op: impl Parser<char, BinOp, Error = Error> + 'static,
    over: impl Parser<char, Expr, Error = Error> + Clone + 'static,
) -> BoxedParser<'static, char, Expr, Error> {
    let over_op = over
        .clone()
        .then(space.clone())
        .then(op)
        .then(space)
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

pub fn expr(space: EParser<Space>, prefixed: EParser<Expr>) -> EParser<Expr> {
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
        space.clone(),
        op_prec_0,
        right_assoc(
            space.clone(),
            op_prec_1,
            left_assoc(
                space.clone(),
                op_prec_2,
                left_assoc(
                    space.clone(),
                    op_prec_3,
                    left_assoc(space, op_prec_4, prefixed),
                ),
            ),
        ),
    )
}

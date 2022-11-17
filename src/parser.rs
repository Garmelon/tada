use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Expr, Ident, Lit, NumLit, TableConstr, TableConstrElem, TableLit, TableLitElem};

fn ident() -> impl Parser<char, Ident, Error = Simple<char>> {
    text::ident().padded().map(Ident)
}

fn num_lit_radix_digits(
    radix: u32,
    max_digits: usize,
) -> impl Parser<char, (i64, String), Error = Simple<char>> + Clone {
    filter(move |c: &char| c.is_digit(radix) || *c == '_')
        .repeated()
        .at_least(1)
        .try_map(move |chars, span| {
            let text = chars.iter().copied().collect::<String>();
            let digits = chars.into_iter().filter(|c| *c != '_').collect::<String>();
            if digits.len() > max_digits {
                return Err(Simple::custom(span, "number out of range"));
            }
            let number = u64::from_str_radix(&digits, radix).unwrap();
            if number > i64::MAX as u64 {
                return Err(Simple::custom(span, "number out of range"));
            }
            Ok((number as i64, text))
        })
}

fn num_lit_bin_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> + Clone {
    // u64::MAX in binary is 0b_1111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111
    // 63 digits are necessary to represent the full range.
    const MAX_BIN_DIGITS: usize = 63;

    just("0b").ignore_then(num_lit_radix_digits(2, MAX_BIN_DIGITS))
}

fn num_lit_dec_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> + Clone {
    // u64::MAX in decimal is 9_223_372_036_854_775_807
    // 19 digits are necessary to represent the full range.
    const MAX_DEC_DIGITS: usize = 19;

    num_lit_radix_digits(10, MAX_DEC_DIGITS)
}

fn num_lit_hex_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> + Clone {
    // u64::MAX in hexadecimal is 0x_7fff_ffff_ffff_ffff
    // 16 digits are necessary to represent the full range.
    const MAX_HEX_DIGITS: usize = 16;

    just("0x").ignore_then(num_lit_radix_digits(16, MAX_HEX_DIGITS))
}

fn num_lit() -> impl Parser<char, NumLit, Error = Simple<char>> + Clone {
    (num_lit_bin_digits().map(|(num, text)| NumLit::Bin(num, text)))
        .or(num_lit_hex_digits().map(|(num, text)| NumLit::Hex(num, text)))
        .or(num_lit_dec_digits().map(|(num, text)| NumLit::Dec(num, text)))
        .padded()
}

fn table_lit_elem(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, TableLitElem, Error = Simple<char>> {
    let positional = expr.clone().map(|e| TableLitElem::Positional(Box::new(e)));
    let named = ident()
        .then_ignore(just(":"))
        .then(expr)
        .map(|(n, e)| TableLitElem::Named(n, Box::new(e)));

    positional.or(named)
}

fn table_lit_elems(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, (Vec<TableLitElem>, bool), Error = Simple<char>> {
    let trailing_comma = just(',').or_not().map(|o| o.is_some());

    table_lit_elem(expr)
        .separated_by(just(','))
        .then(trailing_comma)
        .padded()
}

fn table_lit(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, TableLit, Error = Simple<char>> {
    just("'{")
        .ignore_then(table_lit_elems(expr))
        .then_ignore(just("}"))
        .padded()
        .map(|(elems, trailing_comma)| TableLit {
            elems,
            trailing_comma,
        })
}

fn string_lit() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    filter(|_| false).to(String::new()) // TODO Implement
}

fn lit(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, Lit, Error = Simple<char>> {
    let nil = text::keyword("nil").to(Lit::Nil);
    let r#true = text::keyword("true").to(Lit::Bool(true));
    let r#false = text::keyword("false").to(Lit::Bool(false));
    let num = num_lit().map(Lit::Num);
    let string = string_lit().map(Lit::String);
    let table = table_lit(expr).map(Lit::Table);

    nil.or(r#true)
        .or(r#false)
        .or(num)
        .or(string)
        .or(table)
        .padded()
}

fn table_constr_elem(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, TableConstrElem, Error = Simple<char>> {
    let lit = table_lit_elem(expr.clone()).map(TableConstrElem::Lit);
    let indexed = (just('[').ignore_then(expr.clone()).then_ignore(just(']')))
        .padded()
        .then_ignore(just(':'))
        .then(expr)
        .map(|(i, v)| TableConstrElem::Indexed(Box::new(i), Box::new(v)));

    lit.or(indexed)
}

fn table_constr_elems(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, (Vec<TableConstrElem>, bool), Error = Simple<char>> {
    let trailing_comma = just(',').or_not().map(|o| o.is_some());

    table_constr_elem(expr)
        .separated_by(just(','))
        .then(trailing_comma)
        .padded()
}

fn table_constr(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, TableConstr, Error = Simple<char>> {
    just("{")
        .ignore_then(table_constr_elems(expr))
        .then_ignore(just("}"))
        .padded()
        .map(|(elems, trailing_comma)| TableConstr {
            elems,
            trailing_comma,
        })
}

pub fn parser() -> impl Parser<char, TableConstr, Error = Simple<char>> {
    let expr = num_lit().map(|num| Expr::Lit(Lit::Num(num)));
    table_constr(expr).then_ignore(end())
}

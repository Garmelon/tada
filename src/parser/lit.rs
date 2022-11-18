//! Corresponds to `ast::lit`.

use chumsky::prelude::*;

use crate::ast::{Expr, Lit, NumLit, NumLitStr, StringLit, TableLit, TableLitElem};
use crate::builtin::Builtin;

use super::basic::{ident, space, Error};

fn builtin_lit() -> impl Parser<char, Builtin, Error = Error> {
    just('\'').ignore_then(choice((
        text::keyword("get").to(Builtin::Get),
        text::keyword("set").to(Builtin::Set),
        text::keyword("getraw").to(Builtin::GetRaw),
        text::keyword("setraw").to(Builtin::SetRaw),
        text::keyword("getmeta").to(Builtin::GetMeta),
        text::keyword("setmeta").to(Builtin::SetMeta),
        text::keyword("scope").to(Builtin::Scope),
        text::keyword("arg").to(Builtin::Arg),
        text::keyword("destructure").to(Builtin::Destructure),
    )))
}

fn num_lit_str_radix(radix: u32) -> impl Parser<char, (i64, NumLitStr), Error = Error> + Clone {
    // Minimum amount of digits required to represent i64::MAX. The rest of this
    // code assumes that any value that can be represented using this amount of
    // digits fits into an u64.
    let max_digits = match radix {
        2 => 63,
        10 => 19,
        16 => 16,
        _ => panic!("unsupported radix"),
    };

    // Representations of i64::MAX.
    let max_value = match radix {
        2 => "0b_1111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111",
        10 => "9_223_372_036_854_775_807",
        16 => "0x_7fff_ffff_ffff_ffff",
        _ => panic!("unsupported radix"),
    };

    let constructor = match radix {
        2 => NumLitStr::Bin,
        10 => NumLitStr::Dec,
        16 => NumLitStr::Hex,
        _ => panic!("unsupported radix"),
    };

    filter(move |c: &char| c.is_digit(radix) || *c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .try_map(move |s, span| {
            let digits = s.chars().filter(|c| *c != '_').collect::<String>();
            if digits.is_empty() {
                let msg = "integer literal needs to contain at least one digit";
                return Err(Simple::custom(span, msg));
            } else if digits.len() > max_digits {
                let msg = format!("integer literal too large, the maximum value is {max_value}");
                return Err(Simple::custom(span, msg));
            }

            let value = u64::from_str_radix(&digits, radix).unwrap();
            if value <= i64::MAX as u64 {
                Ok((value as i64, constructor(s)))
            } else {
                let msg = format!("integer literal too large, the maximum value is {max_value}");
                Err(Simple::custom(span, msg))
            }
        })
}

pub fn num_lit() -> impl Parser<char, NumLit, Error = Error> + Clone {
    (just("0b").ignore_then(num_lit_str_radix(2)))
        .or(just("0x").ignore_then(num_lit_str_radix(16)))
        .or(num_lit_str_radix(10))
        .map_with_span(|(value, str), span| NumLit { value, str, span })
}

pub fn string_lit() -> impl Parser<char, StringLit, Error = Error> {
    // TODO Parse string literals
    filter(|_| false).map(|_| unreachable!())
}

pub fn table_lit_elem(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableLitElem, Error = Error> {
    let positional = expr
        .clone()
        .map(|value| TableLitElem::Positional(Box::new(value)));

    let named = ident()
        .then(space())
        .then_ignore(just(':'))
        .then(space())
        .then(expr)
        .map_with_span(|(((name, s0), s1), value), span| TableLitElem::Named {
            name,
            s0,
            s1,
            value: Box::new(value),
            span,
        });

    named.or(positional)
}

pub fn table_lit(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableLit, Error = Error> {
    let elem = space()
        .then(table_lit_elem(expr))
        .then(space())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space()).or_not();

    let elems = elem.separated_by(just(',')).then(trailing_comma);

    just("'{")
        .ignore_then(elems)
        .then_ignore(just('}'))
        .map_with_span(|(elems, trailing_comma), span| TableLit {
            elems,
            trailing_comma,
            span,
        })
}

pub fn lit(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Lit, Error = Error> {
    let nil = text::keyword("nil").map_with_span(|_, span| Lit::Nil(span));
    let r#true = text::keyword("true").map_with_span(|_, span| Lit::Bool(true, span));
    let r#false = text::keyword("false").map_with_span(|_, span| Lit::Bool(false, span));
    let builtin = builtin_lit().map_with_span(Lit::Builtin);
    let num = num_lit().map(Lit::Num);
    let string = string_lit().map(Lit::String);
    let table = table_lit(expr).map(Lit::Table);

    nil.or(r#true)
        .or(r#false)
        .or(builtin)
        .or(num)
        .or(string)
        .or(table)
}

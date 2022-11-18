use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Expr, Ident, Lit, NumLit, NumLitStr, Space, StringLit, TableLit, TableLitElem};
use crate::span::Span;

type Error = Simple<char, Span>;

// This would probably look a lot nicer with type_alias_impl_trait:
// https://github.com/rust-lang/rust/issues/63063

fn space() -> impl Parser<char, Space, Error = Error> {
    // TODO Parse comments
    text::whitespace().map_with_span(|(), span| Space {
        comment: vec![],
        span,
    })
}

fn ident() -> impl Parser<char, Ident, Error = Error> {
    text::ident().map_with_span(|name, span| Ident { name, span })
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

fn num_lit() -> impl Parser<char, NumLit, Error = Error> + Clone {
    (just("0b").ignore_then(num_lit_str_radix(2)))
        .or(just("0x").ignore_then(num_lit_str_radix(16)))
        .or(num_lit_str_radix(10))
        .map_with_span(|(value, str), span| NumLit { value, str, span })
}

fn string_lit() -> impl Parser<char, StringLit, Error = Error> {
    // TODO Parse string literals
    filter(|_| false).to(unreachable!())
}

fn table_lit_elem(
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
        .map(|(((name, s0), s1), value)| TableLitElem::Named {
            name,
            s0,
            s1,
            value: Box::new(value),
        });

    positional.or(named)
}

fn table_lit(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableLit, Error = Error> {
    let elem = space()
        .then(table_lit_elem(expr))
        .then(space())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(",").ignore_then(space()).or_not();

    let elems = elem.separated_by(just(",")).then(trailing_comma);

    just("'{")
        .ignore_then(elems)
        .then_ignore(just("}"))
        .map_with_span(|(elems, trailing_comma), span| TableLit {
            elems,
            trailing_comma,
            span,
        })
}

pub fn parser() -> impl Parser<char, TableLit, Error = Error> {
    let expr = num_lit().map(|n| Expr::Lit(Lit::Num(n)));
    table_lit(expr).padded().then_ignore(end())
}

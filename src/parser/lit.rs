//! Corresponds to `ast::lit`.

use chumsky::prelude::*;

use crate::ast::{
    Expr, Ident, Lit, NumLit, NumLitStr, Space, StringLit, StringLitElem, TableLit, TableLitElem,
};
use crate::builtin::Builtin;

use super::basic::{EParser, Error};

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

fn num_lit_str_radix(radix: u32) -> impl Parser<char, (i64, NumLitStr), Error = Error> {
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

fn num_lit() -> impl Parser<char, NumLit, Error = Error> {
    (just("0b").ignore_then(num_lit_str_radix(2)))
        .or(just("0x").ignore_then(num_lit_str_radix(16)))
        .or(num_lit_str_radix(10))
        .map_with_span(|(value, str), span| NumLit { value, str, span })
}

fn string_lit_elem() -> impl Parser<char, StringLitElem, Error = Error> {
    let plain = filter(|c: &char| !matches!(c, '\\' | '"' | '\t' | '\r' | '\n'))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(StringLitElem::Plain);

    // The maximum unicode codepoint is 10ffff, which has 6 digits.
    let unicode_char = filter(|c: &char| c.is_ascii_hexdigit())
        .repeated()
        .at_least(1)
        .at_most(6)
        .collect::<String>()
        .try_map(|str, span| {
            let msg = "not a valid unicode code point";
            let n = u32::from_str_radix(&str, 16).unwrap();
            let c: char = n.try_into().map_err(|_| Simple::custom(span, msg))?;
            Ok(c)
        });
    let unicode = just("\\u{")
        .ignore_then(unicode_char)
        .then_ignore(just('}'))
        .map(StringLitElem::Unicode);
    let backslash = just("\\\\").to(StringLitElem::Backslash);
    let double_quote = just("\\\"").to(StringLitElem::DoubleQuote);
    let tab = just("\\t").to(StringLitElem::Tab);
    let carriage_return = just("\\r").to(StringLitElem::CarriageReturn);
    let newline = just("\\n").to(StringLitElem::Newline);

    plain
        .or(unicode)
        .or(backslash)
        .or(double_quote)
        .or(tab)
        .or(carriage_return)
        .or(newline)
}

fn string_lit() -> impl Parser<char, StringLit, Error = Error> {
    string_lit_elem()
        .repeated()
        .delimited_by(just('"'), just('"'))
        .map_with_span(|elems, span| StringLit { elems, span })
}

pub fn table_lit_elem(
    space: EParser<Space>,
    ident: EParser<Ident>,
    expr: EParser<Expr>,
) -> EParser<TableLitElem> {
    let positional = expr
        .clone()
        .map(|value| TableLitElem::Positional(Box::new(value)));

    let named = ident
        .then(space.clone())
        .then_ignore(just(':'))
        .then(space)
        .then(expr)
        .map_with_span(|(((name, s0), s1), value), span| TableLitElem::Named {
            name,
            s0,
            s1,
            value: Box::new(value),
            span,
        });

    named.or(positional).boxed()
}

fn table_lit(
    space: EParser<Space>,
    table_lit_elem: EParser<TableLitElem>,
) -> impl Parser<char, TableLit, Error = Error> {
    let elem = space
        .clone()
        .then(table_lit_elem)
        .then(space.clone())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(',').ignore_then(space).or_not();

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

pub fn lit(space: EParser<Space>, table_lit_elem: EParser<TableLitElem>) -> EParser<Lit> {
    let nil = text::keyword("nil").map_with_span(|_, span| Lit::Nil(span));
    let r#true = text::keyword("true").map_with_span(|_, span| Lit::Bool(true, span));
    let r#false = text::keyword("false").map_with_span(|_, span| Lit::Bool(false, span));
    let builtin = builtin_lit().map_with_span(Lit::Builtin);
    let num = num_lit().map(Lit::Num);
    let string = string_lit().map(Lit::String);
    let table = table_lit(space, table_lit_elem).map(Lit::Table);

    nil.or(r#true)
        .or(r#false)
        .or(builtin)
        .or(num)
        .or(string)
        .or(table)
        .boxed()
}

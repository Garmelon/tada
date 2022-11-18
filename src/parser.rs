use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{
    Expr, Ident, Lit, NumLit, NumLitStr, Space, StringLit, TableConstr, TableConstrElem, TableLit,
    TableLitElem,
};
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
    filter(|_| false).map(|_| unreachable!())
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

    named.or(positional)
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

fn lit(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Lit, Error = Error> {
    let nil = text::keyword("nil").map_with_span(|_, span| Lit::Nil(span));
    let r#true = text::keyword("true").map_with_span(|_, span| Lit::Bool(true, span));
    let r#false = text::keyword("false").map_with_span(|_, span| Lit::Bool(false, span));
    let num = num_lit().map(Lit::Num);
    let string = string_lit().map(Lit::String);
    let table = table_lit(expr).map(Lit::Table);

    nil.or(r#true).or(r#false).or(num).or(string).or(table)
}

fn table_constr_elem(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableConstrElem, Error = Error> {
    let lit = table_lit_elem(expr.clone()).map(TableConstrElem::Lit);

    let indexed = just("[")
        .ignore_then(space())
        .then(expr.clone())
        .then(space())
        .then_ignore(just("]"))
        .then(space())
        .then_ignore(just(":"))
        .then(space())
        .then(expr)
        .map_with_span(
            |(((((s0, index), s1), s2), s3), value), span| TableConstrElem::Indexed {
                s0,
                index: Box::new(index),
                s1,
                s2,
                s3,
                value: Box::new(value),
                span,
            },
        );

    indexed.or(lit)
}

fn table_constr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, TableConstr, Error = Error> {
    let elem = space()
        .then(table_constr_elem(expr))
        .then(space())
        .map(|((s0, elem), s1)| (s0, elem, s1));

    let trailing_comma = just(",").ignore_then(space()).or_not();

    let elems = elem.separated_by(just(",")).then(trailing_comma);

    just("{")
        .ignore_then(elems)
        .then_ignore(just("}"))
        .map_with_span(|(elems, trailing_comma), span| TableConstr {
            elems,
            trailing_comma,
            span,
        })
}

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

fn atom_var(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    just("[")
        .ignore_then(space())
        .then(expr)
        .then(space())
        .then_ignore(just("]"))
        .map_with_span(|((s0, index), s1), span| Expr::Var {
            s0,
            index: Box::new(index),
            s1,
            span,
        })
}

fn atom_var_assign(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    just("[")
        .ignore_then(space())
        .then(expr.clone())
        .then(space())
        .then_ignore(just("]"))
        .then(space())
        .then_ignore(just("="))
        .then(space())
        .then(expr)
        .map_with_span(
            |(((((s0, index), s1), s2), s3), value), span| Expr::VarAssign {
                s0,
                index: Box::new(index),
                s1,
                s2,
                s3,
                value: Box::new(value),
                span,
            },
        )
}

fn atom_var_ident_assign(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    ident()
        .then(space())
        .then_ignore(just("="))
        .then(space())
        .then(expr)
        .map(|(((name, s0), s1), value)| Expr::VarIdentAssign {
            name,
            s0,
            s1,
            value: Box::new(value),
        })
}

fn atom(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    let lit = lit(expr.clone()).map(Expr::Lit);
    let paren = atom_paren(expr.clone());
    let table_constr = table_constr(expr.clone()).map(Expr::TableConstr);
    let var = atom_var(expr.clone());
    let var_ident = ident().map(Expr::VarIdent);
    let var_assign = atom_var_assign(expr.clone());
    let var_ident_assign = atom_var_ident_assign(expr);

    lit.or(paren)
        .or(table_constr)
        .or(var_assign)
        .or(var)
        .or(var_ident_assign)
        .or(var_ident)
}

fn expr(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> {
    atom(expr)
}

pub fn parser() -> impl Parser<char, Expr, Error = Error> {
    recursive(expr).padded().then_ignore(end())
}

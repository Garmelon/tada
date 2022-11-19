//! Corresponds to `ast::basic`.

use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Ident, Line, Space};
use crate::span::Span;

pub type Error = Simple<char, Span>;

// TODO https://github.com/rust-lang/rust/issues/63063

fn inline() -> impl Parser<char, (), Error = Error> + Clone {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .to(())
}

fn newline() -> impl Parser<char, (), Error = Error> + Clone {
    just('\n').to(())
}

fn line() -> impl Parser<char, Line, Error = Error> + Clone {
    let empty = newline().to(Line::Empty);

    let comment = just('#')
        .ignore_then(take_until(newline()))
        .map(|(s, _)| s)
        .collect::<String>()
        .map(Line::Comment);

    empty.or(comment)
}

pub fn space() -> BoxedParser<'static, char, Space, Error> {
    inline()
        .ignore_then(line())
        .repeated()
        .then_ignore(inline())
        .map_with_span(|lines, span| Space { lines, span })
        .boxed()
}

pub fn ident() -> BoxedParser<'static, char, Ident, Error> {
    text::ident()
        .try_map(|name, span| {
            if matches!(
                &name as &str,
                "nil" | "true" | "false" | "local" | "function" | "not" | "and" | "or"
            ) {
                Err(Simple::custom(span, "identifier uses reserved name"))
            } else {
                Ok(Ident { name, span })
            }
        })
        .boxed()
}

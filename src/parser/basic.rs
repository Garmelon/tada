//! Corresponds to `ast::basic`.

use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Ident, Line, Space};
use crate::span::Span;

pub type Error = Simple<char, Span>;

// TODO https://github.com/rust-lang/rust/issues/63063

pub fn inline() -> impl Parser<char, (), Error = Error> + Clone {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .to(())
}

pub fn newline() -> impl Parser<char, (), Error = Error> + Clone {
    just('\n').to(())
}

pub fn line() -> impl Parser<char, Line, Error = Error> + Clone {
    let empty = newline().to(Line::Empty);

    let comment = just('#')
        .ignore_then(take_until(newline()))
        .map(|(s, _)| s)
        .collect::<String>()
        .map(Line::Comment);

    empty.or(comment)
}

pub fn space() -> impl Parser<char, Space, Error = Error> + Clone {
    inline()
        .ignore_then(line())
        .repeated()
        .then_ignore(inline())
        .map_with_span(|lines, span| Space { lines, span })
}

pub fn ident() -> impl Parser<char, Ident, Error = Error> + Clone {
    text::ident().try_map(|name, span| {
        if matches!(
            &name as &str,
            "nil" | "true" | "false" | "local" | "not" | "and" | "or"
        ) {
            Err(Simple::custom(span, "identifier uses reserved name"))
        } else {
            Ok(Ident { name, span })
        }
    })
}

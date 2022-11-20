//! Corresponds to `ast::basic`.

use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Ident, Line, Space};
use crate::span::Span;

pub type Error = Simple<char, Span>;
pub type EParser<O> = BoxedParser<'static, char, O, Error>;

fn inline() -> impl Parser<char, (), Error = Error> {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
        .repeated()
        .to(())
}

fn newline() -> impl Parser<char, (), Error = Error> {
    just('\n').to(())
}

fn line() -> impl Parser<char, Line, Error = Error> {
    let empty = newline().to(Line::Empty);

    let comment = just('#')
        .ignore_then(take_until(newline()))
        .map(|(s, _)| s)
        .collect::<String>()
        .map(Line::Comment);

    empty.or(comment)
}

pub fn space() -> EParser<Space> {
    inline()
        .ignore_then(line())
        .repeated()
        .then_ignore(inline())
        .map_with_span(|lines, span| Space { lines, span })
        .boxed()
}

pub fn ident() -> EParser<Ident> {
    text::ident()
        .try_map(|name, span| {
            if matches!(
                &name as &str,
                "nil" | "true" | "false" | "not" | "and" | "or" | "local" | "function" | "module"
            ) {
                Err(Simple::custom(span, "identifier uses reserved name"))
            } else {
                Ok(Ident { name, span })
            }
        })
        .boxed()
}

pub fn local(space: EParser<Space>) -> EParser<Option<Space>> {
    text::keyword("local").ignore_then(space).or_not().boxed()
}

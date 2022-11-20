//! Corresponds to `ast::basic`.

use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Ident, Line, Separated, Space};
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

// This function is more of a utility function. Because of this and to keep the
// code nicer, I have decided that the rules specified in the `parser` module
// don't apply to it.
pub fn separated_by<E: 'static, S1: 'static, S2: 'static>(
    elem: impl Parser<char, E, Error = Error> + Clone + 'static,
    separator: impl Parser<char, S1, Error = Error> + 'static,
    trailing_separator: impl Parser<char, S2, Error = Error> + 'static,
) -> EParser<Separated<E, S1, S2>> {
    elem.clone()
        .then(separator.then(elem).repeated())
        .then(trailing_separator.or_not())
        .or_not()
        .map_with_span(|s, span| match s {
            Some(((first_elem, last_elems), trailing)) => Separated::NonEmpty {
                first_elem,
                last_elems,
                trailing,
                span,
            },
            None => Separated::Empty(span),
        })
        .boxed()
}

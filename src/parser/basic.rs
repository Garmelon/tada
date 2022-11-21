//! Corresponds to `ast::basic`.

use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{BoundedSeparated, Ident, Line, Space};
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
pub fn bounded_separated<E: 'static>(
    space: impl Parser<char, Space, Error = Error> + Clone + 'static,
    start: impl Parser<char, (), Error = Error> + 'static,
    end: impl Parser<char, (), Error = Error> + 'static,
    separator: impl Parser<char, (), Error = Error> + 'static,
    elem: impl Parser<char, E, Error = Error> + Clone + 'static,
) -> EParser<BoundedSeparated<E>> {
    start
        .ignore_then(space.clone())
        .then(
            elem.clone()
                .then(space.clone())
                .then_ignore(separator)
                .then(space.clone())
                .repeated(),
        )
        .then(elem.then(space).or_not())
        .then_ignore(end)
        .map_with_span(|((s0, first_elems), last_elem), span| {
            let mut space_before_elem = s0;
            let mut elems = vec![];
            for ((elem, s1), s2) in first_elems {
                elems.push((space_before_elem, elem, s1));
                space_before_elem = s2;
            }
            let trailing = if let Some((elem, s1)) = last_elem {
                elems.push((space_before_elem, elem, s1));
                None
            } else {
                Some(space_before_elem)
            };
            BoundedSeparated {
                elems,
                trailing,
                span,
            }
        })
        .boxed()
}

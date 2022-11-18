use chumsky::prelude::*;

use crate::ast::{Ident, Space};
use crate::span::Span;

type Error = Simple<char, Span>;

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

pub fn parser() -> impl Parser<char, Ident, Error = Error> {
    ident().padded().then_ignore(end())
}

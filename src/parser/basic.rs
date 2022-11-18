//! Corresponds to `ast::basic`.

use chumsky::prelude::*;

use crate::ast::{Ident, Space};
use crate::span::Span;

pub type Error = Simple<char, Span>;

// TODO https://github.com/rust-lang/rust/issues/63063

pub fn space() -> impl Parser<char, Space, Error = Error> {
    // TODO Parse comments
    text::whitespace().map_with_span(|(), span| Space {
        comment: vec![],
        span,
    })
}

pub fn ident() -> impl Parser<char, Ident, Error = Error> {
    // TODO Forbid keywords
    text::ident().map_with_span(|name, span| Ident { name, span })
}

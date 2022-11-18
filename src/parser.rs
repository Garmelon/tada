use chumsky::prelude::*;
use chumsky::text::whitespace;

use crate::ast::Space;
use crate::span::Span;

type Error = Simple<char, Span>;

fn space() -> impl Parser<char, Space, Error = Error> {
    // TODO Parse comments
    whitespace().map_with_span(|(), span| Space {
        comment: vec![],
        span,
    })
}

pub fn parser() -> impl Parser<char, Space, Error = Error> {
    space().then_ignore(end())
}

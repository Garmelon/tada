use chumsky::prelude::*;

use crate::ast::Ident;

fn ident() -> impl Parser<char, Ident, Error = Simple<char>> {
    text::ident().map(Ident)
}

pub fn parser() -> impl Parser<char, Ident, Error = Simple<char>> {
    ident().padded().then_ignore(end())
}

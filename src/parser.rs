//! Parse the ast over at [`crate::ast`].
//!
//! Rules of thumb:
//! - Everything `pub` should return a [`BoxedParser`].
//! - Everything not used outside a module should not be `pub`. It can always be
//!   made public later.

// TODO Turn multiple calls to subparsers into clone-s

mod basic;
mod expr;
mod func_defs;
mod lit;
mod prefix;
mod suffix;
mod table_constr;
mod table_destr;
mod var;

use chumsky::prelude::*;

use crate::ast::Expr;

use self::basic::Error;

pub fn parser() -> impl Parser<char, Expr, Error = Error> {
    recursive(expr::expr).padded().then_ignore(end())
}

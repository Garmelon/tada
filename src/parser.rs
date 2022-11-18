mod basic;
mod expr;
mod lit;
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

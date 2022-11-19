use chumsky::prelude::*;

use crate::ast::{Expr, FuncDef};

use super::basic::Error;

pub fn func_def(
    expr: impl Parser<char, Expr, Error = Error>,
) -> BoxedParser<'static, char, FuncDef, Error> {
    todo().boxed()
}

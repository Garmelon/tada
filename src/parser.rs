//! Parse the ast over at [`crate::ast`].
//!
//! # Rules
//!
//! - Public parser functions must return [`basic::EParser`].
//! - Public parser functions must receive public subparsers via their arguments.
//! - Each public parser function must be called exactly once, inside this file.
//! - Non-public parser functions may receive and return `impl Parser<...>`.
//!
//! # Warning
//!
//! `clippy::redundant_clone` has stopped working in this module and its
//! submodules. I have no idea why.

// TODO https://github.com/rust-lang/rust/issues/63063
// TODO Find out why clippy::redundant_clone stopped working

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
    let space = basic::space();
    let ident = basic::ident();
    let local = basic::local(space.clone());
    let table_pattern = table_destr::table_pattern(space.clone(), ident.clone());

    let expr = recursive(|expr| {
        let expr = expr.boxed();

        let table_lit_elem = lit::table_lit_elem(space.clone(), ident.clone(), expr.clone());
        let lit = lit::lit(space.clone(), table_lit_elem.clone());
        let var = var::var(space.clone(), ident.clone(), local.clone(), expr.clone());
        let table_constr = table_constr::table_constr(space.clone(), table_lit_elem, expr.clone());
        let table_destr = table_destr::table_destr(
            space.clone(),
            local.clone(),
            table_pattern.clone(),
            expr.clone(),
        );
        let func_def = func_defs::func_def(
            space.clone(),
            ident.clone(),
            local,
            table_pattern,
            expr.clone(),
        );
        let atom = expr::atom(
            space.clone(),
            lit,
            var,
            table_constr.clone(),
            table_destr,
            func_def,
            expr.clone(),
        );
        let suffixed = suffix::suffixed(space.clone(), ident, table_constr, atom, expr);
        let prefixed = prefix::prefixed(space.clone(), suffixed);

        expr::expr(space, prefixed)
    });

    expr.padded().then_ignore(end())
}

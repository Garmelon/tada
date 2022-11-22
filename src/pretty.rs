// TODO Remove this and print whitespace and comments properly
#![allow(unused_variables)]

use pretty::{Pretty, RcAllocator};

mod basic;
mod call;
mod expr;
mod field;
mod func_def;
mod lit;
mod program;
mod table_constr;
mod table_destr;
mod var;

const NEST_DEPTH: isize = 4;

pub fn pretty_to_string<P: Pretty<'static, RcAllocator>>(p: P, width: usize) -> String {
    let mut out = vec![];
    p.pretty(&RcAllocator)
        .render(width, &mut out)
        .expect("p could not be rendered");
    String::from_utf8(out).expect("p created non-utf8 string")
}

use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Program;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Program {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Expr {
                s0,
                expr,
                s1,
                span: _,
            } => expr.pretty(allocator),
            Self::Module {
                s0,
                s1,
                elems,
                s2,
                span: _,
            } => allocator
                .text("module")
                .append(allocator.line())
                .append(allocator.line())
                .append(elems.pretty(
                    allocator,
                    |e| e.pretty(allocator),
                    |(s0, s1)| allocator.text(",").append(allocator.line()),
                    |s| allocator.text(","),
                )),
        }
    }
}

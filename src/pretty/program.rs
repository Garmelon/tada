use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Program;

impl<'a, D> Pretty<'a, D> for Program
where
    D: DocAllocator<'a>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Expr {
                s0,
                expr,
                s1,
                span: _,
            } => expr.pretty(allocator),

            Self::Module { s0, elems, span: _ } => {
                allocator.text("module").append(allocator.line()).append(
                    allocator
                        .intersperse(
                            elems.elems.into_iter().map(|(s0, elem, s1)| {
                                allocator.line().append(elem.pretty(allocator))
                            }),
                            allocator.text(","),
                        )
                        .append(elems.trailing.map(|s| allocator.text(","))),
                )
            }
        }
    }
}

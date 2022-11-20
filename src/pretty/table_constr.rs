use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{TableConstr, TableConstrElem};

use super::NEST_DEPTH;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for TableConstrElem {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Lit(lit) => lit.pretty(allocator),
            Self::Indexed {
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span: _,
            } => index
                .pretty(allocator)
                .brackets()
                .append(allocator.text(": "))
                .append(value.pretty(allocator)),
        }
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for TableConstr {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        self.elems
            .pretty(
                allocator,
                |e| allocator.line().append(e.pretty(allocator)),
                |(s0, s1)| allocator.text(","),
                |s| allocator.text(","),
            )
            .nest(NEST_DEPTH)
            .append(allocator.line())
            .braces()
            .group()
    }
}

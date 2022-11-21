use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{TableConstr, TableConstrElem};

impl<'a, D> Pretty<'a, D> for TableConstrElem
where
    D: DocAllocator<'a>,
    D::Doc: Clone,
{
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

impl<'a, D> Pretty<'a, D> for TableConstr
where
    D: DocAllocator<'a>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        self.0.pretty(
            allocator,
            allocator.text("{"),
            allocator.text("}"),
            allocator.text(","),
            |e| e.pretty(allocator),
        )
    }
}

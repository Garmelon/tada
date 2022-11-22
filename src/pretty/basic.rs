use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{BoundedSeparated, Ident, Space};

use super::NEST_DEPTH;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Ident {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator.text(self.name)
    }
}

impl<E> BoundedSeparated<E> {
    pub fn pretty<'a, D, FE>(
        self,
        allocator: &'a D,
        start: DocBuilder<'a, D>,
        end: DocBuilder<'a, D>,
        separator: DocBuilder<'a, D>,
        elem_pretty: FE,
    ) -> DocBuilder<'a, D>
    where
        D: DocAllocator<'a>,
        D::Doc: Clone,
        FE: Fn(E) -> DocBuilder<'a, D>,
    {
        let elems_empty = self.elems.is_empty();
        allocator
            .intersperse(
                self.elems
                    .into_iter()
                    .map(|(s0, elem, s1)| allocator.line().append(elem_pretty(elem))),
                separator.clone(),
            )
            .append(self.trailing.filter(|_| !elems_empty).map(|s| separator))
            .nest(NEST_DEPTH)
            .append(allocator.line())
            .enclose(start, end)
            .group()
    }
}

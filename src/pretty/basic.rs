use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{BoundedSeparated, Ident, Separated};

use super::NEST_DEPTH;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Ident {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator.text(self.name)
    }
}

impl<E, S1, S2> Separated<E, S1, S2> {
    pub fn pretty<'a, D, FE, FS1, FS2>(
        self,
        allocator: &'a D,
        elem_to_doc: FE,
        separator_to_doc: FS1,
        trailing_separator_to_doc: FS2,
    ) -> DocBuilder<'a, D>
    where
        D: DocAllocator<'a>,
        FE: Fn(E) -> DocBuilder<'a, D>,
        FS1: Fn(S1) -> DocBuilder<'a, D>,
        FS2: Fn(S2) -> DocBuilder<'a, D>,
    {
        match self {
            Self::Empty(_) => allocator.nil(),
            Self::NonEmpty {
                first_elem,
                last_elems,
                trailing,
                span: _span,
            } => elem_to_doc(first_elem)
                .append(
                    allocator.concat(
                        last_elems
                            .into_iter()
                            .map(|(s, e)| separator_to_doc(s).append(elem_to_doc(e))),
                    ),
                )
                .append(
                    trailing
                        .map(trailing_separator_to_doc)
                        .unwrap_or_else(|| allocator.nil()),
                ),
        }
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
        allocator
            .intersperse(
                self.elems
                    .into_iter()
                    .map(|(s0, elem, s1)| allocator.line().append(elem_pretty(elem))),
                separator.clone(),
            )
            .append(self.trailing.map(|s| separator))
            .nest(NEST_DEPTH)
            .append(allocator.line())
            .enclose(start, end)
            .group()
    }
}

use pretty::{DocAllocator, DocBuilder};

use crate::ast::Separated;

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
        FE: Fn(&'a D, E) -> DocBuilder<'a, D>,
        FS1: Fn(&'a D, S1) -> DocBuilder<'a, D>,
        FS2: Fn(&'a D, S2) -> DocBuilder<'a, D>,
    {
        match self {
            Separated::Empty(_) => allocator.nil(),
            Separated::NonEmpty {
                first_elem,
                last_elems,
                trailing,
                span: _span,
            } => elem_to_doc(allocator, first_elem)
                .append(allocator.concat(last_elems.into_iter().map(|(s, e)| {
                    separator_to_doc(allocator, s).append(elem_to_doc(allocator, e))
                })))
                .append(
                    trailing
                        .map(|s| trailing_separator_to_doc(allocator, s))
                        .unwrap_or_else(|| allocator.nil()),
                ),
        }
    }
}

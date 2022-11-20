use pretty::RcDoc;

use crate::ast::Separated;

impl<E, S1, S2> Separated<E, S1, S2> {
    pub fn to_doc<FE, FS1, FS2>(
        &self,
        elem_to_doc: FE,
        separator_to_doc: FS1,
        trailing_separator_to_doc: FS2,
    ) -> RcDoc
    where
        FE: Fn(&E) -> RcDoc,
        FS1: Fn(&S1) -> RcDoc,
        FS2: Fn(&S2) -> RcDoc,
    {
        match self {
            Separated::Empty(_) => RcDoc::nil(),
            Separated::NonEmpty {
                first_elem,
                last_elems,
                trailing,
                span: _span,
            } => elem_to_doc(first_elem)
                .append(RcDoc::concat(
                    last_elems
                        .iter()
                        .map(|(s, e)| separator_to_doc(s).append(elem_to_doc(e))),
                ))
                .append(
                    trailing
                        .as_ref()
                        .map(trailing_separator_to_doc)
                        .unwrap_or_else(RcDoc::nil),
                ),
        }
    }
}

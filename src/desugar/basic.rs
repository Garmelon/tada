use crate::ast::BoundedSeparated;

impl<E> BoundedSeparated<E> {
    pub fn desugar(self, desugar_elem: impl Fn(E) -> (E, bool)) -> (Self, bool) {
        let mut desugared = false;
        let mut elems = vec![];
        for (s0, elem, s1) in self.elems {
            if desugared {
                elems.push((s0, elem, s1));
            } else {
                let (elem, elem_desugared) = desugar_elem(elem);
                desugared = desugared || elem_desugared;
                elems.push((s0, elem, s1));
            }
        }

        let new = Self {
            elems,
            trailing: self.trailing,
            span: self.span,
        };
        (new, desugared)
    }
}

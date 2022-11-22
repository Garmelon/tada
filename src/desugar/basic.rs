use crate::ast::{BoundedSeparated, Space};

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

    pub fn map<E2>(self, f: impl Fn(E) -> E2) -> BoundedSeparated<E2> {
        let elems = self
            .elems
            .into_iter()
            .map(|(s0, e, s1)| (s0, f(e), s1))
            .collect::<Vec<_>>();

        BoundedSeparated {
            elems,
            trailing: self.trailing,
            span: self.span,
        }
    }

    pub fn remove_map<E1, E2>(
        self,
        f: impl Fn(E) -> Result<E1, E2>,
    ) -> (BoundedSeparated<E1>, Vec<(Space, E2, Space)>) {
        let mut kept = vec![];
        let mut removed = vec![];
        for (s0, elem, s1) in self.elems {
            match f(elem) {
                Ok(elem) => kept.push((s0, elem, s1)),
                Err(elem) => removed.push((s0, elem, s1)),
            }
        }
        let new = BoundedSeparated {
            elems: kept,
            trailing: self.trailing,
            span: self.span,
        };
        (new, removed)
    }
}

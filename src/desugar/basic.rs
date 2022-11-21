use crate::ast::Separated;

impl<E, S1, S2> Separated<E, S1, S2> {
    pub fn desugar_elem(self, desugar_elem: impl Fn(E) -> (E, bool)) -> (Self, bool) {
        match self {
            Self::Empty(span) => (Self::Empty(span), false),

            Self::NonEmpty {
                first_elem,
                last_elems,
                trailing,
                span,
            } => {
                let (new_first_elem, mut desugared) = desugar_elem(first_elem);
                let mut new_last_elems = vec![];
                for (separator, elem) in last_elems {
                    if desugared {
                        new_last_elems.push((separator, elem));
                    } else {
                        let (elem, elem_desugared) = desugar_elem(elem);
                        desugared = desugared || elem_desugared;
                        new_last_elems.push((separator, elem));
                    }
                }

                let new = Self::NonEmpty {
                    first_elem: new_first_elem,
                    last_elems: new_last_elems,
                    trailing,
                    span,
                };
                (new, desugared)
            }
        }
    }
}

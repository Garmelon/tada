use crate::span::{HasSpan, Span};

use super::{Space, TableLitElem};

#[derive(Debug, Clone)]
pub struct Program {
    pub elems: Vec<(Space, TableLitElem, Space)>,
    /// `Some` if there is a trailing comma, `None` otherwise.
    pub trailing_comma: Option<Space>,
    pub span: Span,
}

impl HasSpan for Program {
    fn span(&self) -> Span {
        self.span
    }
}

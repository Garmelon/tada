use crate::span::{HasSpan, Span};

use super::{Expr, Space, TableLitElem};

#[derive(Debug, Clone)]
pub enum TableConstrElem {
    /// See [`TableLitElem`].
    Lit(TableLitElem),

    /// `[a]: b`
    ///
    /// Structure: `[ s0 index s1 ] s2 : s3 value`
    Indexed {
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for TableConstrElem {
    fn span(&self) -> Span {
        match self {
            TableConstrElem::Lit(lit) => lit.span(),
            TableConstrElem::Indexed { span, .. } => *span,
        }
    }
}

/// `{ a, b, foo: c, [d]: e }`
#[derive(Debug, Clone)]
pub struct TableConstr {
    pub elems: Vec<(Space, TableConstrElem, Space)>,
    /// `Some` if there is a trailing comma, `None` otherwise.
    pub trailing_comma: Option<Space>,
    pub span: Span,
}

impl HasSpan for TableConstr {
    fn span(&self) -> Span {
        self.span
    }
}

use crate::span::{HasSpan, Span};

use super::{BoundedSeparated, Expr, Ident, Space, TableLitElem};

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
            Self::Lit(lit) => lit.span(),
            Self::Indexed { span, .. } => *span,
        }
    }
}

impl TableConstrElem {
    pub fn positional(value: Box<Expr>) -> Self {
        Self::Lit(TableLitElem::Positional(value))
    }

    pub fn named(name: Ident, value: Box<Expr>, span: Span) -> Self {
        Self::Lit(TableLitElem::named(name, value, span))
    }

    pub fn indexed(index: Box<Expr>, value: Box<Expr>, span: Span) -> Self {
        Self::Indexed {
            s0: Space::empty(span),
            index,
            s1: Space::empty(span),
            s2: Space::empty(span),
            s3: Space::empty(span),
            value,
            span,
        }
    }
}

/// `{ a, b, foo: c, [d]: e }`
#[derive(Debug, Clone)]
pub struct TableConstr(pub BoundedSeparated<TableConstrElem>);

impl HasSpan for TableConstr {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl TableConstr {
    pub fn expr(self) -> Expr {
        Expr::TableConstr(self)
    }
}

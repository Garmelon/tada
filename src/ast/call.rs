use crate::span::{HasSpan, Span};

use super::{Expr, Space, TableConstr};

#[derive(Debug, Clone)]
pub enum Call {
    /// `a(b)`
    ///
    /// Structure: `expr s0 ( s1 arg s2 )`
    Arg {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        arg: Box<Expr>,
        s2: Space,
        span: Span,
    },

    /// `a()`
    ///
    /// Structure: `expr s0 ( s1 )`
    NoArg {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        span: Span,
    },

    /// `a{..}`
    ///
    /// Structure: `expr s0 constr`
    Constr {
        expr: Box<Expr>,
        s0: Space,
        constr: TableConstr,
        span: Span,
    },
}

impl HasSpan for Call {
    fn span(&self) -> Span {
        match self {
            Self::Arg { span, .. } => *span,
            Self::NoArg { span, .. } => *span,
            Self::Constr { span, .. } => *span,
        }
    }
}

impl Call {
    pub fn arg(base: Box<Expr>, arg: Box<Expr>, span: Span) -> Self {
        Self::Arg {
            expr: base,
            s0: Space::empty(span),
            s1: Space::empty(span),
            arg,
            s2: Space::empty(span),
            span,
        }
    }

    pub fn no_arg(base: Box<Expr>, span: Span) -> Self {
        Self::NoArg {
            expr: base,
            s0: Space::empty(span),
            s1: Space::empty(span),
            span,
        }
    }

    pub fn constr(base: Box<Expr>, constr: TableConstr, span: Span) -> Self {
        Self::Constr {
            expr: base,
            s0: Space::empty(span),
            constr,
            span,
        }
    }

    pub fn expr(self) -> Expr {
        Expr::Call(self)
    }
}

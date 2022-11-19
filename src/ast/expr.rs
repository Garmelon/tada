use std::fmt::{self, write};

use crate::span::{HasSpan, Span};

use super::{Call, Field, FuncDef, Lit, Space, TableConstr, TableDestr, Var};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `==`
    Eq,
    /// `!=`
    Neq,
    /// `and`
    And,
    /// `or`
    Or,
}

#[derive(Clone)]
pub enum Expr {
    Lit(Lit),
    Call(Call),
    Field(Field),
    Var(Var),
    TableConstr(TableConstr),
    TableDestr(TableDestr),
    FuncDef(FuncDef),

    /// `(a)`
    ///
    /// Structure: `( s0 inner s1 )`
    Paren {
        s0: Space,
        inner: Box<Expr>,
        s1: Space,
        span: Span,
    },

    /// `-a`
    ///
    /// Structure: `- s0 expr`
    Neg {
        minus: Span,
        s0: Space,
        expr: Box<Expr>,
        span: Span,
    },

    /// `not a`
    ///
    /// Structure: `'not' s0 expr`
    Not {
        not: Span,
        s0: Space,
        expr: Box<Expr>,
        span: Span,
    },

    /// - `a + b`
    /// - `a == b`
    /// - `a and b`
    ///
    /// Structure: `left s0 op s1 right`
    BinOp {
        left: Box<Expr>,
        s0: Space,
        op: BinOp,
        s1: Space,
        right: Box<Expr>,
        span: Span,
    },
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(lit) => {
                f.write_str("Expr::Lit(")?;
                lit.fmt(f)?;
                f.write_str(")")
            }
            Self::Call(call) => {
                f.write_str("Expr::Call(")?;
                call.fmt(f)?;
                f.write_str(")")
            }
            Self::Field(field) => {
                f.write_str("Expr::Field(")?;
                field.fmt(f)?;
                f.write_str(")")
            }
            Self::Var(var) => {
                f.write_str("Expr::Var(")?;
                var.fmt(f)?;
                f.write_str(")")
            }
            Self::TableConstr(constr) => {
                f.write_str("Expr::TableConstr(")?;
                constr.fmt(f)?;
                f.write_str(")")
            }
            Self::TableDestr(destr) => {
                f.write_str("Expr::TableDestr(")?;
                destr.fmt(f)?;
                f.write_str(")")
            }
            Self::FuncDef(def) => {
                f.write_str("Expr::FuncDef(")?;
                def.fmt(f)?;
                f.write_str(")")
            }
            Self::Paren {
                s0,
                inner,
                s1,
                span,
            } => f
                .debug_struct("Expr::Paren")
                .field("s0", s0)
                .field("inner", inner)
                .field("s1", s1)
                .field("span", span)
                .finish(),
            Self::Neg {
                minus,
                s0,
                expr,
                span,
            } => f
                .debug_struct("Expr::Neg")
                .field("minus", minus)
                .field("s0", s0)
                .field("expr", expr)
                .field("span", span)
                .finish(),
            Self::Not {
                not,
                s0,
                expr,
                span,
            } => f
                .debug_struct("Expr::Not")
                .field("not", not)
                .field("s0", s0)
                .field("expr", expr)
                .field("span", span)
                .finish(),
            Self::BinOp {
                left,
                s0,
                op,
                s1,
                right,
                span,
            } => f
                .debug_struct("Expr::BinOp")
                .field("left", left)
                .field("s0", s0)
                .field("op", op)
                .field("s1", s1)
                .field("right", right)
                .field("span", span)
                .finish(),
        }
    }
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Lit(lit) => lit.span(),
            Expr::Call(call) => call.span(),
            Expr::Field(field) => field.span(),
            Expr::Var(var) => var.span(),
            Expr::TableConstr(constr) => constr.span(),
            Expr::TableDestr(destr) => destr.span(),
            Expr::FuncDef(def) => def.span(),
            Expr::Paren { span, .. } => *span,
            Expr::Neg { span, .. } => *span,
            Expr::Not { span, .. } => *span,
            Expr::BinOp { span, .. } => *span,
        }
    }
}

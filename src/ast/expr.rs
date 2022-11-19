use crate::span::{HasSpan, Span};

use super::basic::Space;
use super::call::Call;
use super::field::Field;
use super::func_def::FuncDef;
use super::lit::Lit;
use super::table_constr::TableConstr;
use super::table_destr::TableDestr;
use super::var::Var;

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

#[derive(Debug, Clone)]
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

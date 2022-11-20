use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Expr;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Expr {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Lit(lit) => lit.pretty(allocator),
            Self::Call(call) => allocator.text("<call>"),
            Self::Field(field) => allocator.text("<field>"),
            Self::Var(var) => allocator.text("<var>"),
            Self::TableConstr(constr) => allocator.text("<onstr>"),
            Self::TableDestr(destr) => allocator.text("<destr>"),
            Self::FuncDef(def) => allocator.text("<def>"),
            Self::Paren {
                s0,
                inner,
                s1,
                span: _,
            } => inner.pretty(allocator).parens(),

            // TODO Check whether parentheses are necessary
            Self::Neg {
                minus: _,
                s0,
                expr,
                span: _,
            } => allocator.text("-").append(expr.pretty(allocator)),

            // TODO Check whether parentheses are necessary
            Self::Not {
                not: _,
                s0,
                expr,
                span: _,
            } => allocator.text("not ").append(expr.pretty(allocator)),

            Self::BinOp {
                left,
                s0,
                op,
                s1,
                right,
                span: _,
            } => allocator.text("<binop>"),
        }
    }
}

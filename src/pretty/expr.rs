use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Expr;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Expr {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Expr::Lit(lit) => allocator.text("<lit>"),
            Expr::Call(call) => allocator.text("<call>"),
            Expr::Field(field) => allocator.text("<field>"),
            Expr::Var(var) => allocator.text("<var>"),
            Expr::TableConstr(constr) => allocator.text("<onstr>"),
            Expr::TableDestr(destr) => allocator.text("<destr>"),
            Expr::FuncDef(def) => allocator.text("<def>"),
            Expr::Paren {
                s0,
                inner,
                s1,
                span: _,
            } => inner.pretty(allocator).parens(),

            // TODO Check whether parentheses are necessary
            Expr::Neg {
                minus: _,
                s0,
                expr,
                span: _,
            } => allocator.text("-").append(expr.pretty(allocator)),

            // TODO Check whether parentheses are necessary
            Expr::Not {
                not: _,
                s0,
                expr,
                span: _,
            } => allocator.text("not ").append(expr.pretty(allocator)),

            Expr::BinOp {
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

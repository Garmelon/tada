use pretty::RcDoc;

use crate::ast::Expr;

impl Expr {
    pub fn to_doc(&self) -> RcDoc {
        match self {
            Expr::Lit(lit) => RcDoc::text("<lit>"),
            Expr::Call(call) => RcDoc::text("<call>"),
            Expr::Field(field) => RcDoc::text("<field>"),
            Expr::Var(var) => RcDoc::text("<var>"),
            Expr::TableConstr(constr) => RcDoc::text("<onstr>"),
            Expr::TableDestr(destr) => RcDoc::text("<destr>"),
            Expr::FuncDef(def) => RcDoc::text("<def>"),
            Expr::Paren {
                s0,
                inner,
                s1,
                span: _,
            } => RcDoc::text("(").append(inner.to_doc()).append(")"),

            // TODO Check whether parentheses are necessary
            Expr::Neg {
                minus: _,
                s0,
                expr,
                span: _,
            } => RcDoc::text("-").append(expr.to_doc()),

            // TODO Check whether parentheses are necessary
            Expr::Not {
                not: _,
                s0,
                expr,
                span: _,
            } => RcDoc::text("not ").append(expr.to_doc()),

            Expr::BinOp {
                left,
                s0,
                op,
                s1,
                right,
                span: _,
            } => RcDoc::text("<binop>"),
        }
    }
}

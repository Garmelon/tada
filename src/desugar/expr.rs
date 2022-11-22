use crate::ast::Expr;

impl Expr {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Lit(lit) => {
                let (lit, desugared) = lit.desugar();
                (Self::Lit(lit), desugared)
            }

            Self::Call(call) => call.desugar(),
            Self::Field(field) => field.desugar(),
            Self::Var(var) => var.desugar(),
            Self::TableConstr(constr) => constr.desugar(),
            Self::TableDestr(destr) => destr.desugar(),
            Self::FuncDef(def) => def.desugar(),

            Self::Paren {
                s0,
                inner,
                s1,
                span,
            } => (
                Self::Paren {
                    s0,
                    inner,
                    s1,
                    span,
                },
                false,
            ), // TODO Implement

            Self::Neg {
                minus,
                s0,
                expr,
                span,
            } => (
                Self::Neg {
                    minus,
                    s0,
                    expr,
                    span,
                },
                false,
            ), // TODO Implement

            Self::Not {
                not,
                s0,
                expr,
                span,
            } => (
                Self::Not {
                    not,
                    s0,
                    expr,
                    span,
                },
                false,
            ), // TODO Implement

            Self::BinOp {
                left,
                s0,
                op,
                s1,
                right,
                span,
            } => (
                Self::BinOp {
                    left,
                    s0,
                    op,
                    s1,
                    right,
                    span,
                },
                false,
            ), // TODO Implement
        }
    }
}

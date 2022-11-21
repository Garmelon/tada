use crate::ast::Expr;

impl Expr {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Lit(lit) => {
                let (lit, desugared) = lit.desugar();
                (Self::Lit(lit), desugared)
            }

            Self::Call(call) => (Self::Call(call), false), // TODO Implement
            Self::Field(field) => (Self::Field(field), false), // TODO Implement
            Self::Var(var) => (Self::Var(var), false),     // TODO Implement
            Self::TableConstr(constr) => (Self::TableConstr(constr), false), // TODO Implement
            Self::TableDestr(destr) => (Self::TableDestr(destr), false), // TODO Implement
            Self::FuncDef(def) => (Self::FuncDef(def), false), // TODO Implement

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

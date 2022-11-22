use crate::ast::{BinOp, BoundedSeparated, Call, Expr, Lit, TableConstrElem};
use crate::builtin::Builtin;

impl Expr {
    pub fn desugar(self) -> (Self, bool) {
        match self {
            Self::Lit(lit) => {
                let (lit, desugared) = lit.desugar();
                (lit.expr(), desugared)
            }

            Self::Call(call) => call.desugar(),
            Self::Field(field) => field.desugar(),
            Self::Var(var) => var.desugar(),
            Self::TableConstr(constr) => constr.desugar(),
            Self::TableDestr(destr) => destr.desugar(),
            Self::FuncDef(def) => def.desugar(),

            Self::Paren {
                s0: _,
                inner,
                s1: _,
                span: _,
            } => (*inner, true),

            Self::Neg {
                minus,
                s0: _,
                expr,
                span,
            } => {
                let new = Call::arg(Lit::Builtin(Builtin::Neg, minus).expr().boxed(), expr, span);
                (new.expr(), true)
            }

            Self::Not {
                not,
                s0: _,
                expr,
                span,
            } => {
                let new = Call::arg(Lit::Builtin(Builtin::Not, not).expr().boxed(), expr, span);
                (new.expr(), true)
            }

            Self::BinOp {
                left,
                s0: _,
                op,
                s1: _,
                right,
                span,
            } => {
                let builtin = match op {
                    BinOp::Mul => Builtin::Mul,
                    BinOp::Div => Builtin::Div,
                    BinOp::Mod => Builtin::Mod,
                    BinOp::Add => Builtin::Add,
                    BinOp::Sub => Builtin::Sub,
                    BinOp::Eq => Builtin::Eq,
                    BinOp::Neq => Builtin::Ne,
                    BinOp::Gt => Builtin::Gt,
                    BinOp::Ge => Builtin::Ge,
                    BinOp::Lt => Builtin::Lt,
                    BinOp::Le => Builtin::Le,
                    BinOp::And => Builtin::And,
                    BinOp::Or => Builtin::Or,
                };
                let constr = BoundedSeparated::new(span)
                    .then(TableConstrElem::positional(left))
                    .then(TableConstrElem::positional(right))
                    .table_constr();
                let new = Call::constr(Lit::Builtin(builtin, span).expr().boxed(), constr, span);
                (new.expr(), true)
            }
        }
    }
}

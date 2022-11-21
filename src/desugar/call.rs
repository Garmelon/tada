use chumsky::Span;

use crate::ast::{Call, Expr, Lit, Space};
use crate::span::HasSpan;

impl Call {
    pub fn desugar(self) -> (Expr, bool) {
        match self {
            Self::Arg {
                expr,
                s0,
                s1,
                arg,
                s2,
                span,
            } => {
                let new = Expr::Call(Self::Arg {
                    expr,
                    s0,
                    s1,
                    arg,
                    s2,
                    span,
                });
                (new, false) // TODO Implement
            }

            Self::NoArg { expr, s0, s1, span } => {
                let (expr, desugared) = expr.desugar();
                if desugared {
                    let new = Expr::Call(Self::NoArg {
                        expr: Box::new(expr),
                        s0,
                        s1,
                        span,
                    });
                    return (new, true);
                }

                let arg_span = s1.span().at_start();
                let arg = Expr::Lit(Lit::Nil(arg_span));
                let new = Expr::Call(Self::Arg {
                    expr: Box::new(expr),
                    s0,
                    s1,
                    arg: Box::new(arg),
                    s2: Space::empty(arg_span.at_end()),
                    span,
                });
                (new, true)
            }

            Self::Constr {
                expr,
                s0,
                constr,
                span,
            } => {
                let (expr, desugared) = expr.desugar();
                if desugared {
                    let new = Expr::Call(Self::Constr {
                        expr: Box::new(expr),
                        s0,
                        constr,
                        span,
                    });
                    return (new, true);
                }

                let arg = Expr::TableConstr(constr);
                let arg_span = arg.span();
                let new = Expr::Call(Self::Arg {
                    expr: Box::new(expr),
                    s0,
                    s1: Space::empty(arg_span.at_start()),
                    arg: Box::new(arg),
                    s2: Space::empty(arg_span.at_end()),
                    span,
                });
                (new, true)
            }
        }
    }
}

use crate::ast::{Call, Expr, Ident, Lit, Separated, Space, TableLit, TableLitElem};
use crate::span::HasSpan;

// TODO Add span for just the parentheses to ast, or limit span to parentheses

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
                let (expr, desugared) = expr.desugar();
                if desugared {
                    let new = Expr::Call(Self::Arg {
                        expr: Box::new(expr),
                        s0,
                        s1,
                        arg,
                        s2,
                        span,
                    });
                    return (new, true);
                }

                let (arg, desugared) = arg.desugar();
                if desugared {
                    let new = Expr::Call(Self::Arg {
                        expr: Box::new(expr),
                        s0,
                        s1,
                        arg: Box::new(arg),
                        s2,
                        span,
                    });
                    return (new, true);
                }

                let call = TableLitElem::Named {
                    name: Ident::new("call", span),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    value: Box::new(expr),
                    span,
                };
                let arg = TableLitElem::Named {
                    name: Ident::new("arg", span),
                    s0: Space::empty(span),
                    s1: Space::empty(span),
                    value: Box::new(arg),
                    span,
                };
                let elems = Separated::NonEmpty {
                    first_elem: call,
                    last_elems: vec![((Space::empty(span), Space::empty(span)), arg)],
                    trailing: None,
                    span,
                };
                let new = Expr::Lit(Lit::Table(TableLit {
                    s0: Space::empty(span),
                    elems,
                    s1: Space::empty(span),
                    span,
                }));
                (new, true)
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

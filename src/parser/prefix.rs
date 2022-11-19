//! Corresponds to `Expr::Neg` and `Expr::Not` in `ast::expr`

use chumsky::prelude::*;

use crate::ast::{Expr, Space};
use crate::span::{HasSpan, Span};

use super::basic::{space, Error};

enum Prefix {
    /// See [`Expr::Neg`].
    Neg { minus: Span, s0: Space },

    /// See [`Expr::Not`].
    Not { not: Span, s0: Space },
}

impl Prefix {
    fn into_expr(self, span: Span, expr: Expr) -> Expr {
        let expr = Box::new(expr);
        match self {
            Self::Neg { minus, s0 } => Expr::Neg {
                minus,
                s0,
                expr,
                span,
            },
            Self::Not { not, s0 } => Expr::Not {
                not,
                s0,
                expr,
                span,
            },
        }
    }
}

fn prefix_neg() -> impl Parser<char, Prefix, Error = Error> + Clone {
    just('-')
        .map_with_span(|_, span| span)
        .then(space())
        .map(|(minus, s0)| Prefix::Neg { minus, s0 })
}

fn prefix_not() -> impl Parser<char, Prefix, Error = Error> + Clone {
    text::keyword("not")
        .map_with_span(|_, span| span)
        .then(space())
        .map(|(not, s0)| Prefix::Not { not, s0 })
}

pub fn prefixed(
    suffixed: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Expr, Error = Error> + Clone {
    let prefix = prefix_neg()
        .or(prefix_not())
        .map_with_span(|prefix, span| (prefix, span));

    prefix
        .repeated()
        .then(suffixed)
        .foldr(|(prefix, span), expr| prefix.into_expr(expr.span().join(span), expr))
}

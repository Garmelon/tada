use crate::ast::{
    BoundedSeparated, Expr, Field, Ident, TableConstr, TableConstrElem, TableLitElem,
};
use crate::span::HasSpan;

impl TableConstr {
    pub fn desugar(self) -> (Expr, bool) {
        let span = self.span();

        let (elems, setters) = self.0.remove_map(|e| match e {
            TableConstrElem::Lit(lit) => Ok(lit),
            TableConstrElem::Indexed {
                s0: _,
                index,
                s1: _,
                s2: _,
                s3: _,
                value,
                span,
            } => Err((index, value, span)),
        });

        let mut expr = BoundedSeparated::new(span)
            .then(TableLitElem::named(
                Ident::new("raw", span),
                elems.table_lit().lit().expr().boxed(),
                span,
            ))
            .table_lit()
            .lit()
            .expr();

        for (_, (index, value, span), _) in setters {
            expr = Field::assign(expr.boxed(), index, value, span).expr();
        }

        (expr, true)
    }
}

use crate::ast::{
    BoundedSeparated, Expr, Field, Ident, Line, TableConstr, TableConstrElem, TableLitElem,
};
use crate::span::HasSpan;

impl TableConstr {
    pub fn desugar(self) -> (Expr, bool) {
        let span = self.span();

        let (elems, setters) = self.0.remove_map(|e| match e {
            TableConstrElem::Lit(lit) => Ok(lit),
            TableConstrElem::Indexed {
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span,
            } => Err((s0, index, s1, s2, s3, value, span)),
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

        // `sl [ s0 index s1 ] s2 = s3 value sr`
        // -> `expr s0 [ s1 index s2 ] s3 = s4 s5 value`
        for (s0, (s1, index, s2, s3, s4, value, span), s5) in setters {
            expr = Field::Assign {
                expr: expr.boxed(),
                s0,
                s1,
                index,
                s2,
                s3,
                s4: s4.then_line(Line::Empty).then(s5),
                value,
                span,
            }
            .expr();
        }

        (expr, true)
    }
}

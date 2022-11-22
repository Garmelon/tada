use crate::ast::{
    BoundedSeparated, Expr, Field, Ident, Line, Lit, Space, TableConstr, TableConstrElem,
    TableLitElem,
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

        let raw_elem = TableLitElem::Named {
            name: Ident::new("raw", span),
            s0: Space::empty(span),
            s1: Space::empty(span),
            value: Box::new(Expr::Lit(Lit::Table(elems.table_lit()))),
            span,
        };
        let mut expr = Expr::Lit(Lit::Table(
            BoundedSeparated::new(span).then(raw_elem).table_lit(),
        ));

        // `sl [ s0 index s1 ] s2 = s3 value sr`
        // -> `expr s0 [ s1 index s2 ] s3 = s4 s5 value`
        for (s0, (s1, index, s2, s3, s4, value, span), s5) in setters {
            expr = Expr::Field(Field::Assign {
                expr: Box::new(expr),
                s0,
                s1,
                index,
                s2,
                s3,
                s4: s4.then_line(Line::Empty).then(s5),
                value,
                span,
            });
        }

        (expr, true)
    }
}

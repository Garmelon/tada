use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Field;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Field {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Access {
                expr,
                s0,
                s1,
                index,
                s2,
                span: _,
            } => expr
                .pretty(allocator)
                .append(index.pretty(allocator).brackets()),
            Self::Assign {
                expr,
                s0,
                s1,
                index,
                s2,
                s3,
                s4,
                value,
                span: _,
            } => expr
                .pretty(allocator)
                .append(index.pretty(allocator).brackets())
                .append(allocator.text(" = "))
                .append(value.pretty(allocator)),
            Self::AccessIdent {
                expr,
                s0,
                s1,
                ident,
                span: _,
            } => expr
                .pretty(allocator)
                .append(allocator.line_())
                .append(allocator.text("."))
                .append(ident.pretty(allocator))
                .group(),
            Self::AssignIdent {
                expr,
                s0,
                s1,
                ident,
                s2,
                s3,
                value,
                span: _,
            } => expr
                .pretty(allocator)
                .append(allocator.line_())
                .append(allocator.text("."))
                .append(ident.pretty(allocator))
                .append(allocator.text(" = "))
                .append(value.pretty(allocator))
                .group(),
        }
    }
}

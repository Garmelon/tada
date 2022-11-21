use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Var;

impl<'a, D> Pretty<'a, D> for Var
where
    D: DocAllocator<'a>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Access {
                s0,
                index,
                s1,
                span: _,
            } => index.pretty(allocator).brackets(),
            Self::Assign {
                local,
                s0,
                index,
                s1,
                s2,
                s3,
                value,
                span: _,
            } => local
                .map(|s| allocator.text("local "))
                .unwrap_or_else(|| allocator.nil())
                .append(index.pretty(allocator).brackets())
                .append(allocator.text(" = "))
                .append(value.pretty(allocator)),
            Self::AccessIdent(ident) => ident.pretty(allocator),
            Self::AssignIdent {
                local,
                name,
                s0,
                s1,
                value,
                span: _,
            } => local
                .map(|s| allocator.text("local "))
                .unwrap_or_else(|| allocator.nil())
                .append(name.pretty(allocator))
                .append(allocator.text(" = "))
                .append(value.pretty(allocator)),
        }
    }
}

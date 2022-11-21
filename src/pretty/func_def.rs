use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::FuncDef;

impl<'a, D> Pretty<'a, D> for FuncDef
where
    D: DocAllocator<'a>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::AnonNoArg {
                s0,
                s1,
                s2,
                body,
                span: _,
            } => allocator.text("function() ").append(body.pretty(allocator)),

            Self::AnonArg {
                s0,
                s1,
                arg,
                s2,
                s3,
                body,
                span: _,
            } => allocator
                .text("function")
                .append(arg.pretty(allocator).parens())
                .append(allocator.space())
                .append(body.pretty(allocator)),

            Self::AnonDestr {
                s0,
                pattern,
                s1,
                body,
                span: _,
            } => allocator
                .text("function")
                .append(pattern.pretty(allocator))
                .append(allocator.space())
                .append(body.pretty(allocator)),

            Self::NamedNoArg {
                local,
                s0,
                name,
                s1,
                s2,
                s3,
                body,
                span: _,
            } => local
                .map(|s| allocator.text("local "))
                .unwrap_or_else(|| allocator.nil())
                .append(allocator.text("function "))
                .append(name)
                .append(allocator.text("() "))
                .append(body.pretty(allocator)),

            Self::NamedArg {
                local,
                s0,
                name,
                s1,
                s2,
                arg,
                s3,
                s4,
                body,
                span: _,
            } => local
                .map(|s| allocator.text("local "))
                .unwrap_or_else(|| allocator.nil())
                .append(allocator.text("function "))
                .append(name)
                .append(arg.pretty(allocator).parens())
                .append(allocator.space())
                .append(body.pretty(allocator)),

            Self::NamedDestr {
                local,
                s0,
                name,
                s1,
                pattern,
                s2,
                body,
                span: _,
            } => local
                .map(|s| allocator.text("local "))
                .unwrap_or_else(|| allocator.nil())
                .append(allocator.text("function "))
                .append(name)
                .append(pattern.pretty(allocator))
                .append(allocator.space())
                .append(body.pretty(allocator)),
        }
    }
}

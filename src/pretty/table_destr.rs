use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{TableDestr, TablePattern, TablePatternElem};

use super::NEST_DEPTH;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for TablePatternElem {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Positional(ident) => ident.pretty(allocator),
            Self::Named {
                name,
                s0,
                s1,
                ident,
                span: _,
            } => name
                .pretty(allocator)
                .append(allocator.text(": "))
                .append(ident.pretty(allocator)),
        }
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for TablePattern {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        self.elems
            .pretty(
                allocator,
                |a, e| a.line().append(e.pretty(a)),
                |a, (s0, s1)| a.text(","),
                |a, s| a.text(","),
            )
            .nest(NEST_DEPTH)
            .append(allocator.line())
            .braces()
            .group()
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for TableDestr {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        // TODO Handle spaces
        self.local
            .map(|s| allocator.text("local "))
            .unwrap_or_else(|| allocator.nil())
            .append(self.pattern.pretty(allocator))
            .append(allocator.text(" = "))
            .append(self.value.pretty(allocator))
    }
}

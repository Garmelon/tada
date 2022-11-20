use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{Lit, NumLit};

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for NumLit {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator.text(format!("{self:?}"))
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Lit {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Nil(_) => allocator.text("nil"),
            Self::Bool(false, _) => allocator.text("false"),
            Self::Bool(true, _) => allocator.text("true"),
            Self::Builtin(builtin, _) => allocator.text(format!("{builtin:?}")),
            Self::Num(num) => num.pretty(allocator),
            Self::String(string) => allocator.text("<string>"),
            Self::Table(table) => allocator.text("<table>"),
        }
    }
}

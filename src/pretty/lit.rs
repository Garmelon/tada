use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{Lit, NumLit, StringLit, StringLitElem};

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for NumLit {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator.text(format!("{self:?}"))
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for StringLitElem {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Plain(str) => allocator.text(str),
            Self::Unicode(char) => allocator.text(format!("\\u{{{:x}}}", char as u32)),
            Self::Backslash => allocator.text("\\\\"),
            Self::DoubleQuote => allocator.text("\\\""),
            Self::Tab => allocator.text("\\t"),
            Self::CarriageReturn => allocator.text("\\r"),
            Self::Newline => allocator.text("\\n"),
        }
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for StringLit {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator
            .concat(self.elems.into_iter().map(|e| e.pretty(allocator)))
            .enclose(allocator.text("\""), allocator.text("\""))
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
            Self::String(string) => string.pretty(allocator),
            Self::Table(table) => allocator.text("<table>"),
        }
    }
}

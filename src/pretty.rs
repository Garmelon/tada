use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Program;

impl<'a, A: DocAllocator<'a>> Pretty<'a, A> for Program {
    fn pretty(self, allocator: &'a A) -> DocBuilder<'a, A, ()> {
        allocator.text("Hello world")
    }
}

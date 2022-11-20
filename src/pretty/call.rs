use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::Call;

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Call {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Arg {
                expr,
                s0,
                s1,
                arg,
                s2,
                span: _,
            } => expr
                .pretty(allocator)
                .append(arg.pretty(allocator).parens()),
            Self::NoArg {
                expr,
                s0,
                s1,
                span: _,
            } => expr.pretty(allocator).append(allocator.nil().parens()),
            Self::Constr {
                expr,
                s0,
                constr,
                span: _,
            } => expr.pretty(allocator).append(constr.pretty(allocator)),
        }
    }
}

use pretty::RcDoc;

use crate::ast::Program;

impl Program {
    pub fn to_doc(&self) -> RcDoc {
        match self {
            Program::Expr {
                s0,
                expr,
                s1,
                span: _,
            } => RcDoc::nil(),
            Program::Module {
                s0,
                s1,
                elems,
                s2,
                span: _,
            } => RcDoc::text("module"),
        }
    }
}

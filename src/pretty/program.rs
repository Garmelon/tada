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
            } => expr.to_doc(),
            Program::Module {
                s0,
                s1,
                elems,
                s2,
                span: _,
            } => RcDoc::text("module")
                .append(RcDoc::line())
                .append(RcDoc::line())
                .append(elems.to_doc(
                    |e| RcDoc::text("<elem>"),
                    |(s0, s1)| RcDoc::text(",").append(RcDoc::line()),
                    |s| RcDoc::text(","),
                )),
        }
    }
}

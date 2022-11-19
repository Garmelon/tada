mod basic;
mod call;
mod expr;
mod field;
mod func_def;
mod lit;
mod table_constr;
mod table_destr;
mod var;

use crate::span::{HasSpan, Span};

pub use self::basic::*;
pub use self::call::*;
pub use self::expr::*;
pub use self::field::*;
pub use self::func_def::*;
pub use self::lit::*;
pub use self::table_constr::*;
pub use self::table_destr::*;
pub use self::var::*;

#[derive(Debug, Clone)]
pub struct Program {
    pub elems: Vec<(Space, TableLitElem, Space)>,
    /// `Some` if there is a trailing comma, `None` otherwise.
    pub trailing_comma: Option<Space>,
    pub span: Span,
}

impl HasSpan for Program {
    fn span(&self) -> Span {
        self.span
    }
}

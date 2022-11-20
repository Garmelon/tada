use std::fmt;

use crate::builtin::Builtin;
use crate::span::{HasSpan, Span};

use super::{Expr, Ident, Space};

#[derive(Clone)]
pub enum NumLitStr {
    /// - `0b_0001_1011`
    /// - `0b10`
    Bin(String),

    /// - `12_345`
    /// - `7`
    Dec(String),

    /// - `0x_c0_f3`
    /// - `0xB`
    Hex(String),
}

impl fmt::Debug for NumLitStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bin(str) => write!(f, "0b{str}"),
            Self::Dec(str) => write!(f, "{str}"),
            Self::Hex(str) => write!(f, "0x{str}"),
        }
    }
}

/// Positive number literal.
///
/// Possible bases are binary, decimal, hexadecimal. Underscores can be inserted
/// before and after any digit.
#[derive(Clone)]
pub struct NumLit {
    pub value: i64,
    pub str: NumLitStr,
    pub span: Span,
}

impl fmt::Debug for NumLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.str.fmt(f)
    }
}

impl HasSpan for NumLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum StringLitElem {
    /// Normal unescaped characters
    Plain(String),
    /// `\u{xxxx}`
    Unicode(char),
    /// `\\`
    Backslash,
    /// `\'`
    SingleQuote,
    /// `\"'`
    DoubleQuote,
    /// `\t`
    Tab,
    /// `\r`
    CarriageReturn,
    /// `\n`
    Newline,
}

/// - `"Hello world\n"`
/// - `""`
#[derive(Debug, Clone)]
pub struct StringLit {
    elems: Vec<StringLitElem>,
    span: Span,
}

impl HasSpan for StringLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum TableLitElem {
    /// `a`
    Positional(Box<Expr>),

    /// `foo: a`
    ///
    /// Structure: `name s0 : s1 value`
    Named {
        name: Ident,
        s0: Space,
        s1: Space,
        value: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for TableLitElem {
    fn span(&self) -> Span {
        match self {
            TableLitElem::Positional(value) => value.span(),
            TableLitElem::Named { span, .. } => *span,
        }
    }
}

/// `'{ a, foo: b }`
#[derive(Debug, Clone)]
pub struct TableLit {
    pub elems: Vec<(Space, TableLitElem, Space)>,
    /// `Some` if there is a trailing comma, `None` otherwise.
    pub trailing_comma: Option<Space>,
    pub span: Span,
}

impl HasSpan for TableLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub enum Lit {
    /// `nil`
    Nil(Span),

    /// - `true`
    /// - `false`
    Bool(bool, Span),

    /// - `'get`
    /// - `'destructure`
    Builtin(Builtin, Span),

    /// See [`NumLit`].
    Num(NumLit),

    /// See [`StringLit`]
    String(StringLit),

    /// See [`TableLit`].
    Table(TableLit),
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(_) => write!(f, "l#nil"),
            Self::Bool(b, _) => write!(f, "l#{b:?}"),
            Self::Builtin(b, _) => write!(f, "l#{b:?}"),
            Self::Num(n) => write!(f, "l#{n:?}"),
            Self::String(s) => write!(f, "l#{s:?}"),
            Self::Table(t) => {
                write!(f, "l#")?;
                t.fmt(f)
            }
        }
    }
}

impl HasSpan for Lit {
    fn span(&self) -> Span {
        match self {
            Lit::Nil(span) => *span,
            Lit::Bool(_, span) => *span,
            Lit::Builtin(_, span) => *span,
            Lit::Num(n) => n.span(),
            Lit::String(s) => s.span(),
            Lit::Table(t) => t.span(),
        }
    }
}

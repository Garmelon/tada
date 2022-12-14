use std::fmt;

use crate::builtin::Builtin;
use crate::span::{HasSpan, Span};

use super::{BoundedSeparated, Expr, Ident, Space};

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

#[derive(Clone)]
pub enum StringLitElem {
    /// Normal unescaped characters
    Plain(String),
    /// `\u{xxxx}`
    Unicode(char),
    /// `\\`
    Backslash,
    /// `\"'`
    DoubleQuote,
    /// `\t`
    Tab,
    /// `\r`
    CarriageReturn,
    /// `\n`
    Newline,
}

impl fmt::Debug for StringLitElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain(str) => write!(f, "Plain({str:?})"),
            Self::Unicode(char) => write!(f, "Unicode(0x{:x})", *char as u32),
            Self::Backslash => write!(f, "Backslash"),
            Self::DoubleQuote => write!(f, "DoubleQuote"),
            Self::Tab => write!(f, "Tab"),
            Self::CarriageReturn => write!(f, "CarriageReturn"),
            Self::Newline => write!(f, "Newline"),
        }
    }
}

/// - `"Hello world\n"`
/// - `""`
#[derive(Debug, Clone)]
pub struct StringLit {
    pub elems: Vec<StringLitElem>,
    pub span: Span,
}

impl StringLit {
    pub fn from_ident(ident: Ident) -> Self {
        Self {
            elems: vec![StringLitElem::Plain(ident.name)],
            span: ident.span,
        }
    }

    pub fn lit(self) -> Lit {
        Lit::String(self)
    }
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
            Self::Positional(value) => value.span(),
            Self::Named { span, .. } => *span,
        }
    }
}

impl TableLitElem {
    pub fn named(name: Ident, value: Box<Expr>, span: Span) -> Self {
        Self::Named {
            name,
            s0: Space::empty(span),
            s1: Space::empty(span),
            value,
            span,
        }
    }
}

/// `'{ a, foo: b }`
#[derive(Debug, Clone)]
pub struct TableLit(pub BoundedSeparated<TableLitElem>);

impl HasSpan for TableLit {
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl TableLit {
    pub fn lit(self) -> Lit {
        Lit::Table(self)
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
            Self::String(s) => {
                write!(f, "l#")?;
                s.fmt(f)
            }
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
            Self::Nil(span) => *span,
            Self::Bool(_, span) => *span,
            Self::Builtin(_, span) => *span,
            Self::Num(n) => n.span(),
            Self::String(s) => s.span(),
            Self::Table(t) => t.span(),
        }
    }
}

impl Lit {
    pub fn expr(self) -> Expr {
        Expr::Lit(self)
    }
}

use std::fmt;

use crate::span::{HasSpan, Span};

#[derive(Clone)]
pub struct Space {
    pub comment: Vec<(String, Span)>,
    pub span: Span,
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Space").finish()
    }
}

impl HasSpan for Space {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i#{}", self.name)
    }
}

impl HasSpan for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

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
pub enum StringLit {
    /// - `"Hello world\n"`
    /// - `""`
    Inline(String, Span),

    /// ```text
    /// """
    ///     Hello,
    ///     world!
    /// """
    /// ```
    Multiline(String, Span),
}

impl HasSpan for StringLit {
    fn span(&self) -> Span {
        match self {
            StringLit::Inline(_, span) => *span,
            StringLit::Multiline(_, span) => *span,
        }
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
    },
}

impl HasSpan for TableLitElem {
    fn span(&self) -> Span {
        match self {
            TableLitElem::Positional(value) => value.span(),
            TableLitElem::Named { name, value, .. } => name.span().join(value.span()),
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
            Lit::Num(n) => n.span(),
            Lit::String(s) => s.span(),
            Lit::Table(t) => t.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableConstrElem {
    /// See [`TableLitElem`].
    Lit(TableLitElem),

    /// `[a]: b`
    ///
    /// Structure: `[ s0 index s1 ] s2 : s3 value`
    Indexed {
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for TableConstrElem {
    fn span(&self) -> Span {
        match self {
            TableConstrElem::Lit(lit) => lit.span(),
            TableConstrElem::Indexed { span, .. } => *span,
        }
    }
}

/// `{ a, b, foo: c, [d]: e }`
#[derive(Debug, Clone)]
pub struct TableConstr {
    pub elems: Vec<(Space, TableConstrElem, Space)>,
    /// `Some` if there is a trailing comma, `None` otherwise.
    pub trailing_comma: Option<Space>,
    pub span: Span,
}

impl HasSpan for TableConstr {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `==`
    Eq,
    /// `!=`
    Neq,
    /// `and`
    And,
    /// `or`
    Or,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(Lit),

    /// `(a)`
    ///
    /// Structure: `( s0 inner s1 )`
    Paren {
        s0: Space,
        inner: Box<Expr>,
        s1: Space,
        span: Span,
    },

    /// See [`TableConstr`].
    TableConstr(TableConstr),

    /// `[a]`
    ///
    /// Structure: `[ s0 index s1 ]`
    Var {
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        span: Span,
    },

    /// `foo`
    VarIdent(Ident),

    /// `[a] = b`
    ///
    /// Structure: `[ s0 index s1 ] s2 = s3 value`
    VarAssign {
        s0: Space,
        index: Box<Expr>,
        s1: Space,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
        span: Span,
    },

    /// `foo = a`
    ///
    /// Structure: `name s0 = s1 value`
    VarIdentAssign {
        name: Ident,
        s0: Space,
        s1: Space,
        value: Box<Expr>,
    },

    /// `-a`
    ///
    /// Structure: `- s0 expr`
    Neg {
        minus: Span,
        s0: Space,
        expr: Box<Expr>,
    },

    /// `not a`
    ///
    /// Structure: `'not' s0 expr`
    Not {
        not: Span,
        s0: Space,
        expr: Box<Expr>,
    },

    /// `a[b]`
    ///
    /// Structure: `expr s0 [ s1 index s2 ]`
    Field {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        span: Span,
    },

    /// `a.foo`
    ///
    /// Structure: `expr s0 . s1 ident`
    FieldIdent {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        ident: Ident,
    },

    /// `a[b] = c`
    ///
    /// Structure: `expr s0 [ s1 index s2 ] s3 = s4 value`
    FieldAssign {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        index: Box<Expr>,
        s2: Space,
        s3: Space,
        s4: Space,
        value: Box<Expr>,
    },

    /// `a.foo = b`
    ///
    /// Structure: `expr s0 . s1 ident s2 = s3 value`
    FieldIdentAssign {
        expr: Box<Expr>,
        s0: Space,
        s1: Space,
        ident: Ident,
        s2: Space,
        s3: Space,
        value: Box<Expr>,
    },

    /// - `a + b`
    /// - `a == b`
    /// - `a and b`
    ///
    /// Structure: `left s0 op s1 right`
    BinOp {
        left: Box<Expr>,
        s0: Space,
        op: BinOp,
        s1: Space,
        right: Box<Expr>,
    },
}

impl HasSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Lit(lit) => lit.span(),
            Expr::Paren { span, .. } => *span,
            Expr::TableConstr(tcr) => tcr.span(),
            Expr::Var { span, .. } => *span,
            Expr::VarIdent(_) => todo!(),
            Expr::VarAssign { span, .. } => *span,
            Expr::VarIdentAssign { name, value, .. } => name.span().join(value.span()),
            Expr::Neg { minus, expr, .. } => minus.join(expr.span()),
            Expr::Not { not, expr, .. } => not.join(expr.span()),
            Expr::Field { span, .. } => *span,
            Expr::FieldIdent { expr, ident, .. } => expr.span().join(ident.span()),
            Expr::FieldAssign { expr, value, .. } => expr.span().join(value.span()),
            Expr::FieldIdentAssign { expr, value, .. } => expr.span().join(value.span()),
            Expr::BinOp { left, right, .. } => left.span().join(right.span()),
        }
    }
}

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

use std::fmt;

#[derive(Clone)]
pub struct Ident(pub String);

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i#{}", self.0)
    }
}

/// Positive number literal.
///
/// Possible bases are binary, decimal, hexadecimal. Underscores can be inserted
/// before and after any digit.
#[derive(Clone)]
pub enum NumLit {
    /// - `0b_0001_1011`
    /// - `0b10`
    Bin(i64, String),

    /// - `12_345`
    /// - `7`
    Dec(i64, String),

    /// - `0x_c0_f3`
    /// - `0xB`
    Hex(i64, String),
}

impl fmt::Debug for NumLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bin(_, str) => write!(f, "0b{str}"),
            Self::Dec(_, str) => write!(f, "{str}"),
            Self::Hex(_, str) => write!(f, "0x{str}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableLitElem {
    /// `a`
    Positional(Box<Expr>),

    /// `foo: a`
    Named(Ident, Box<Expr>),
}

/// `'{ a, foo: b }`
#[derive(Debug, Clone)]
pub struct TableLit {
    pub elems: Vec<TableLitElem>,
    pub trailing_comma: bool,
}

#[derive(Clone)]
pub enum Lit {
    /// `nil`
    Nil,

    /// - `true`
    /// - `false`
    Bool(bool),

    /// See [`NumLit`].
    Num(NumLit),

    /// - `"foo"`
    /// - `"Hello world!\n"`
    /// - `""`
    String(String),

    /// See [`TableLit`].
    Table(TableLit),
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "l#nil"),
            Self::Bool(b) => write!(f, "l#{b:?}"),
            Self::Num(n) => write!(f, "l#{n:?}"),
            Self::String(s) => write!(f, "l#{s:?}"),
            Self::Table(t) => {
                write!(f, "l#")?;
                t.fmt(f)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableConstrElem {
    /// See [`TableLitElem`].
    Lit(TableLitElem),

    /// `[a]: b`
    Indexed(Box<Expr>, Box<Expr>),
}

/// `{ a, b, foo: c, [d]: e }`
#[derive(Debug, Clone)]
pub struct TableConstr {
    pub elems: Vec<TableConstrElem>,
    pub trailing_comma: bool,
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

    /// See [`TableConstr`].
    TableConstr(TableConstr),

    /// `[a]`
    Var(Box<Expr>),

    /// `foo`
    VarIdent(Ident),

    /// `[a] = b`
    VarAssign(Box<Expr>, Box<Expr>),

    /// `foo = a`
    VarIdentAssign(Ident, Box<Expr>),

    /// `-a`
    Neg(Box<Expr>),

    /// `not a`
    Not(Box<Expr>),

    /// `a[b]`
    Field(Box<Expr>, Box<Expr>),

    /// `a.foo`
    FieldIdent(Box<Expr>, Ident),

    /// `a[b] = c`
    FieldAssign(Box<Expr>, Box<Expr>, Box<Expr>),

    /// `a.foo = b`
    FieldIdentAssign(Box<Expr>, Ident, Box<Expr>),

    /// - `a + b`
    /// - `a == b`
    /// - `a and b`
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

/// The contents of a program file are just a table literal without the
/// surrounding `'{` and `}`.
#[derive(Debug, Clone)]
pub struct Program(pub TableLit);

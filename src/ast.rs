#[derive(Debug, Clone)]
struct Ident(String);

#[derive(Debug, Clone)]
enum NumLit {
    /// - `0b_0001_1011`
    /// - `-0b10`
    Bin(i64, String),

    /// - `12_345`
    /// - `-7`
    Dec(i64, String),

    /// - `0x_c0_f3`
    /// - `-0xB`
    Hex(i64, String),
}

#[derive(Debug, Clone)]
enum TableLitElem {
    /// `a`
    Positional(Box<Expr>),

    /// `foo: a`
    Named(Ident, Box<Expr>),

    /// ```text
    /// # foo
    /// # bar
    /// ```
    MultlineString(String),
}

/// `'{ a, foo: b }`
#[derive(Debug, Clone)]
struct TableLit {
    elems: Vec<TableLitElem>,
    trailing_comma: bool,
}

#[derive(Debug, Clone)]
enum Lit {
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

#[derive(Debug, Clone)]
enum TableConstrElem {
    /// See [`TableLitElem`].
    Lit(TableLitElem),

    /// `[a]: b`
    Indexed(Box<Expr>, Box<Expr>),
}

/// `{ a, b, foo: c, [d]: e }`
#[derive(Debug, Clone)]
struct TableConstr {
    elems: Vec<TableConstrElem>,
    trailing_comma: bool,
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
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
enum Expr {
    Lit(Lit),

    /// See [`TableConstr`].
    TableConstr(TableConstr),

    /// `foo`
    Var(Ident),

    /// `[a]`
    VarExpr(Box<Expr>),

    /// `foo = a`
    VarAssign(Ident, Box<Expr>),

    /// `[a] = b`
    VarExprAssign(Box<Expr>, Box<Expr>),

    /// `-a`
    Neg(Box<Expr>),

    /// `not a`
    Not(Box<Expr>),

    /// `a.foo`
    Field(Box<Expr>, Ident),

    /// `a[b]`
    FieldExpr(Box<Expr>, Box<Expr>),

    /// `a.foo = b`
    FieldAssign(Box<Expr>, Ident, Box<Expr>),

    /// `a[b] = c`
    FieldExprAssign(Box<Expr>, Box<Expr>, Box<Expr>),

    /// - `a + b`
    /// - `a == b`
    /// - `a and b`
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

/// The contents of a program file are just a table literal without the
/// surrounding `'{` and `}`.
#[derive(Debug, Clone)]
struct Program(TableLit);

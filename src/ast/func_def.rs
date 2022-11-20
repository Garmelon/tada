use crate::span::{HasSpan, Span};

use super::{Expr, Ident, Space, TablePattern};

#[derive(Debug, Clone)]
pub enum FuncDef {
    /// `function() a
    ///
    /// Structure: `function s0 ( s1 ) s2 body`
    AnonNoArg {
        s0: Space,
        s1: Space,
        s2: Space,
        body: Box<Expr>,
        span: Span,
    },

    /// `function(foo) a`
    ///
    /// Structure: `function s0 ( s1 arg s2 ) s3 body`
    AnonArg {
        s0: Space,
        s1: Space,
        arg: Ident,
        s2: Space,
        s3: Space,
        body: Box<Expr>,
        span: Span,
    },

    /// `function{..} a`
    ///
    /// Structure: `function s0 pattern s1 body`
    AnonDestr {
        s0: Space,
        pattern: TablePattern,
        s1: Space,
        body: Box<Expr>,
        span: Span,
    },

    /// - `function foo() a`
    /// - `local function foo() a`
    ///
    /// Structure: `local function s0 name s1 ( s2 ) s3 body`
    NamedNoArg {
        local: Option<Space>,
        s0: Space,
        name: Ident,
        s1: Space,
        s2: Space,
        s3: Space,
        body: Box<Expr>,
        span: Span,
    },

    /// - `function foo(bar) a`
    /// - `local function foo(bar) a`
    ///
    /// Structure: `local function s0 name s1 ( s2 arg s3 ) s4 body`
    NamedArg {
        local: Option<Space>,
        s0: Space,
        name: Ident,
        s1: Space,
        s2: Space,
        arg: Ident,
        s3: Space,
        s4: Space,
        body: Box<Expr>,
        span: Span,
    },

    /// `function foo{..} a`
    /// `local function foo{..} a`
    ///
    /// Structure: `local function s0 name s1 pattern s2 body`
    NamedDestr {
        local: Option<Space>,
        s0: Space,
        name: Ident,
        s1: Space,
        pattern: TablePattern,
        s2: Space,
        body: Box<Expr>,
        span: Span,
    },
}

impl HasSpan for FuncDef {
    fn span(&self) -> Span {
        match self {
            Self::AnonNoArg { span, .. } => *span,
            Self::AnonArg { span, .. } => *span,
            Self::AnonDestr { span, .. } => *span,
            Self::NamedNoArg { span, .. } => *span,
            Self::NamedArg { span, .. } => *span,
            Self::NamedDestr { span, .. } => *span,
        }
    }
}

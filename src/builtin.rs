use std::fmt;

/// Built-in operations
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Builtin {
    Get,
    Set,
    GetRaw,
    SetRaw,
    GetMeta,
    SetMeta,
    Scope,
    Arg,
    Destructure,
    Neg,
    Not,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "'get"),
            Self::Set => write!(f, "'set"),
            Self::GetRaw => write!(f, "'getraw"),
            Self::SetRaw => write!(f, "'setraw"),
            Self::GetMeta => write!(f, "'getmeta"),
            Self::SetMeta => write!(f, "'setmeta"),
            Self::Scope => write!(f, "'scope"),
            Self::Arg => write!(f, "'arg"),
            Self::Destructure => write!(f, "'destructure"),
            Self::Neg => write!(f, "'neg"),
            Self::Not => write!(f, "'not"),
            Self::Mul => write!(f, "'mul"),
            Self::Div => write!(f, "'div"),
            Self::Mod => write!(f, "'mod"),
            Self::Add => write!(f, "'add"),
            Self::Sub => write!(f, "'sub"),
            Self::Eq => write!(f, "'eq"),
            Self::Ne => write!(f, "'ne"),
            Self::Gt => write!(f, "'gt"),
            Self::Ge => write!(f, "'ge"),
            Self::Lt => write!(f, "'lt"),
            Self::Le => write!(f, "'le"),
            Self::And => write!(f, "'and"),
            Self::Or => write!(f, "'or"),
        }
    }
}

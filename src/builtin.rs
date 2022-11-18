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
        }
    }
}

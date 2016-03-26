use std::ops::Not;
use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Not for Stone {
    type Output = Self;
    fn not(self) -> Self::Output {
        use self::Stone::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

impl<'a> Not for &'a Stone {
    type Output = Stone;
    fn not(self) -> Self::Output { !*self }
}

impl AsRef<Stone> for Stone {
    fn as_ref(&self) -> &Self { self }
}

impl Display for Stone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

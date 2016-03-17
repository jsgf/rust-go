use std::ops::Not;

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

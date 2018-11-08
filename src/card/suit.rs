use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Copy, Clone, IntoEnumIterator)]
pub enum Suit {
    SPADE,
    HEART,
    DIAMOND,
    CLUB,
}

impl Suit {
    pub fn iter() -> SuitEnumIterator {
        Suit::into_enum_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::SPADE => write!(f, "♠"),
            Suit::HEART => write!(f, "♥"),
            Suit::DIAMOND => write!(f, "♦"),
            Suit::CLUB => write!(f, "♣"),
        }
    }
}

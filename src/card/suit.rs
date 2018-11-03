use std::fmt;
use std::slice::Iter;

#[derive(Copy, Clone)]
pub enum Suit {
    SPADE,
    HEART,
    DIAMOND,
    CLUB,
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        const SUITS: [Suit; 4] = [Suit::SPADE, Suit::HEART, Suit::DIAMOND, Suit::CLUB];
        return SUITS.iter();
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

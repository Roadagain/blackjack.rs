use std::fmt;

#[derive(Copy, Clone)]
pub enum Suit {
    SPADE,
    HEART,
    DIAMOND,
    CLUB,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SPADE => write!(f, "♠"),
            HEART => write!(f, "♥"),
            DIAMOND => write!(f, "♦"),
            CLUB => write!(f, "♣"),
        }
    }
}

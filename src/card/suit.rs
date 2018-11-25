use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, IntoEnumIterator)]
pub enum Suit {
    SPADE,
    HEART,
    DIAMOND,
    CLUB,
}

impl Suit {
    pub fn into_iter() -> SuitEnumIterator {
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

#[cfg(test)]
mod test {
    use super::Suit;

    #[test]
    fn all_suits() {
        let iter = Suit::into_iter();
        let suits: Vec<Suit> = iter.collect();
        let expected = vec![Suit::SPADE, Suit::HEART, Suit::DIAMOND, Suit::CLUB];
        assert_eq!(suits, expected);
    }
}

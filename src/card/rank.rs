use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Copy, Clone, IntoEnumIterator)]
pub enum Rank {
    ACE = 1,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Rank {
    pub fn into_iter() -> RankEnumIterator {
        Self::into_enum_iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::ACE => write!(f, "A"),
            Rank::JACK => write!(f, "J"),
            Rank::QUEEN => write!(f, "Q"),
            Rank::KING => write!(f, "K"),
            n => write!(f, "{}", n as i32),
        }
    }
}

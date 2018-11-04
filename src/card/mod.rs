mod rank;
pub use self::rank::Rank;
mod suit;
pub use self::suit::Suit;
use std::fmt;

pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: i32, suit: Suit) -> Card {
        Card {
            rank: Rank::new(rank),
            suit,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

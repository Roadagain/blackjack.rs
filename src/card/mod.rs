mod rank;
pub use self::rank::Rank;
mod suit;
pub use self::suit::Suit;
mod score;
pub use self::score::Score;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn score(&self) -> Score {
        Score::new(self.rank)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

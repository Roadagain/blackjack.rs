use card::rank::Rank;
use card::suit::Suit;
use std::fmt;

pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: i32) -> Card {
        return Card {
            rank: Rank::new(rank),
            suit: Suit::SPADE,
        };
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

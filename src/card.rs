use std::fmt;

pub struct Card {
    rank: i32,
}

impl Card {
    pub fn new(rank: i32) -> Card {
        return Card { rank: rank };
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.rank {
            1 => write!(f, "A"),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            n => write!(f, "{}", n),
        }
    }
}

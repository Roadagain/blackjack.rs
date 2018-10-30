use std::fmt;

pub struct Rank {
    rank: i32,
}

impl Rank {
    pub fn new(rank: i32) -> Rank {
        return Rank { rank: rank };
    }
}

impl fmt::Display for Rank {
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

use std::fmt;
use std::ops::RangeInclusive;

pub struct Rank {
    rank: i32,
}

impl Rank {
    pub fn new(rank: i32) -> Rank {
        Rank { rank }
    }

    pub const MIN: i32 = 1;
    pub const MAX: i32 = 13;
    pub const RANGE: RangeInclusive<i32> = self::Rank::MIN..=self::Rank::MAX;
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

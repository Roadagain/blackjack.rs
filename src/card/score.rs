use super::Rank;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Score {
    Single { value: u32 },
    Double { min: u32, max: u32 },
}

impl Score {
    pub fn new(rank: Rank) -> Self {
        if rank == Rank::ACE {
            Score::Double { min: 1, max: 11 }
        } else {
            let value = match rank {
                Rank::JACK => 10,
                Rank::QUEEN => 10,
                Rank::KING => 10,
                n => n as u32,
            };
            Score::Single { value }
        }
    }
}

impl Add<Rank> for Score {
    type Output = Score;

    fn add(self, rank: Rank) -> Self {
        let inc = rank as u32;
        match self {
            Score::Single { value } => Score::Single { value: value + inc },
            Score::Double { min, max } => Score::Double {
                min: min + inc,
                max: max + inc,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Rank;
    use super::Score;

    #[test]
    fn jqk_is_ten() {
        let jack_score = Score::new(Rank::JACK);
        match jack_score {
            Score::Single { value } => assert_eq!(value, 10),
            _ => assert!(false),
        }

        let queen_score = Score::new(Rank::QUEEN);
        match queen_score {
            Score::Single { value } => assert_eq!(value, 10),
            _ => assert!(false),
        }

        let king_score = Score::new(Rank::KING);
        match king_score {
            Score::Single { value } => assert_eq!(value, 10),
            _ => assert!(false),
        }
    }

    #[test]
    fn ace_is_one_or_eleven() {
        let ace_score = Score::new(Rank::ACE);
        match ace_score {
            Score::Double { min, max } => {
                assert_eq!(min, 1);
                assert_eq!(max, 11)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn add_rank_to_score() {
        let score = Score::new(Rank::SIX);
        let result = score + Rank::SEVEN;
        match result {
            Score::Single { value } => assert_eq!(value, 13),
            _ => assert!(false),
        }
    }
}

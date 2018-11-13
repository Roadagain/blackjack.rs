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
        if let Score::Single { value } = jack_score {
            assert_eq!(value, 10);
        } else {
            assert!(false);
        }

        let queen_score = Score::new(Rank::QUEEN);
        if let Score::Single { value } = queen_score {
            assert_eq!(value, 10);
        } else {
            assert!(false);
        }

        let king_score = Score::new(Rank::KING);
        if let Score::Single { value } = king_score {
            assert_eq!(value, 10);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ace_is_one_or_eleven() {
        let ace_score = Score::new(Rank::ACE);
        if let Score::Double { min, max } = ace_score {
            assert_eq!(min, 1);
            assert_eq!(max, 11);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn add_rank_to_score() {
        let score = Score::new(Rank::SIX);
        let result = score + Rank::SEVEN;
        if let Score::Single { value } = result {
            assert_eq!(value, 13);
        } else {
            assert!(false)
        }
    }
}

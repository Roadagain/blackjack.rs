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

    fn is_burst(&self) -> bool {
        match *self {
            Score::Single { value } => value > Score::BLACKJACK,
            Score::Double { min, max } => min > Score::BLACKJACK || max > Score::BLACKJACK,
        }
    }

    fn normalize(&self) -> Self {
        match self {
            Score::Double { min, .. } if self.is_burst() => Score::Single { value: *min },
            _ => *self,
        }
    }

    const BLACKJACK: u32 = 21;
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
            }.normalize(),
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
            assert_eq!((min, max), (1, 11));
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

    #[test]
    fn normalize() {
        let safe_single = Score::new(Rank::TEN);
        assert_eq!(safe_single.normalize(), safe_single);

        let bursted_single = Score::Single { value: 30 };
        assert_eq!(bursted_single.normalize(), bursted_single);

        let safe_double = Score::Double {
            min: 11,
            max: Score::BLACKJACK,
        };
        assert_eq!(safe_double.normalize(), safe_double);

        let bursted_double = Score::Double {
            min: 2,
            max: Score::BLACKJACK + 1,
        };
        let expected_normalized = Score::Single { value: 2 };
        assert_eq!(bursted_double.normalize(), expected_normalized);
    }

    #[test]
    fn burst_score() {
        let single_burst = Score::new(Rank::TEN) + Rank::TEN + Rank::TEN;
        assert!(single_burst.is_burst());
        if let Score::Single { value } = single_burst {
            assert_eq!(value, 30);
        } else {
            assert!(false);
        }

        let double_burst = Score::new(Rank::ACE) + Rank::TEN + Rank::TEN;
        assert!(!double_burst.is_burst());
        if let Score::Single { value } = double_burst {
            // 元々DoubleだったものがバーストしてSingleに切り捨てられる
            assert_eq!(value, 21);
        } else {
            assert!(false);
        }
    }
}

use super::Rank;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    min: u32,
    max: u32,
}

impl Score {
    pub fn new(rank: Rank) -> Self {
        let min = match rank {
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            n => n as u32,
        };
        let max = match rank {
            Rank::ACE => 11,
            _ => min,
        };
        Self { min, max }
    }
}

#[cfg(test)]
mod test {
    use super::Rank;
    use super::Score;

    #[test]
    fn jqk_is_ten() {
        let jack_score = Score::new(Rank::JACK);
        assert_eq!(jack_score.min, 10);
        assert_eq!(jack_score.max, 10);

        let queen_score = Score::new(Rank::QUEEN);
        assert_eq!(queen_score.min, 10);
        assert_eq!(queen_score.max, 10);

        let king_score = Score::new(Rank::KING);
        assert_eq!(king_score.min, 10);
        assert_eq!(king_score.max, 10);
    }

    #[test]
    fn ace_is_one_or_eleven() {
        let ace_score = Score::new(Rank::ACE);
        assert_eq!(ace_score.min, 1);
        assert_eq!(ace_score.max, 11);
    }
}

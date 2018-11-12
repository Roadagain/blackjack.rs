use super::Rank;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    score: u32,
}

impl Score {
    pub fn new(rank: Rank) -> Self {
        let score = match rank {
            Rank::ACE => 11,
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            n => n as u32,
        };
        Self { score }
    }
}

#[cfg(test)]
mod test {
    use super::Rank;
    use super::Score;

    #[test]
    fn jqk_is_ten() {
        let jack_score = Score::new(Rank::JACK);
        assert_eq!(jack_score.score, 10);

        let queen_score = Score::new(Rank::QUEEN);
        assert_eq!(queen_score.score, 10);

        let king_score = Score::new(Rank::KING);
        assert_eq!(king_score.score, 10);
    }

    #[test]
    fn ace_is_eleven() {
        let ace_score = Score::new(Rank::ACE);
        assert_eq!(ace_score.score, 11);
    }
}

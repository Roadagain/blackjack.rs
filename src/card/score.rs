use super::Rank;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    value: u32,
    has_ace: bool,
}

impl Score {
    pub fn new(rank: Rank) -> Self {
        let value = match rank {
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            n => n as u32,
        };
        Self {
            value,
            has_ace: rank == Rank::ACE,
        }
    }

    fn is_burst(&self) -> bool {
        self.value > Score::BLACKJACK
    }

    const BLACKJACK: u32 = 21;
}

impl Add<Rank> for Score {
    type Output = Score;

    fn add(self, rank: Rank) -> Self {
        let inc = rank as u32;
        Self {
            value: self.value + inc,
            has_ace: self.has_ace || rank == Rank::ACE,
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
        assert_eq!(jack_score.value, 10);

        let queen_score = Score::new(Rank::QUEEN);
        assert_eq!(queen_score.value, 10);

        let king_score = Score::new(Rank::KING);
        assert_eq!(king_score.value, 10);
    }

    #[test]
    fn add_rank_to_score() {
        let score = Score::new(Rank::SIX);
        let result = score + Rank::SEVEN;
        assert_eq!(result.value, 13);
    }

    #[test]
    fn burst_score() {
        let single_burst = Score::new(Rank::TEN) + Rank::TEN + Rank::TEN;
        assert!(single_burst.is_burst());

        let blackjack_with_ace = Score::new(Rank::ACE) + Rank::TEN + Rank::TEN;
        assert!(!blackjack_with_ace.is_burst());
    }
}

use super::Rank;
use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
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

    fn ace_as_eleven(&self) -> Self {
        if self.has_ace {
            Self {
                value: self.value + 10,
                has_ace: false,
            }
        } else {
            Self {
                value: self.value,
                has_ace: self.has_ace,
            }
        }
    }

    fn is_blackjack(&self) -> bool {
        self.value == Score::BLACKJACK
    }

    fn is_busted(&self) -> bool {
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

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value);

        let special_ace_score = self.ace_as_eleven();
        if self.has_ace && !special_ace_score.is_busted() {
            write!(f, " or {}", special_ace_score.value);
        }

        if self.is_blackjack() || special_ace_score.is_blackjack() {
            write!(f, " Blackjack!")
        } else if self.is_busted() {
            write!(f, " busted!")
        } else {
            write!(f, "")
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
    fn bust_score() {
        let single_bust = Score::new(Rank::TEN) + Rank::TEN + Rank::TEN;
        assert!(single_bust.is_busted());

        let blackjack_with_ace = Score::new(Rank::ACE) + Rank::TEN + Rank::TEN;
        // special score (A is 11) busts for 31, but it must be not busted
        assert!(!blackjack_with_ace.is_busted());
    }

    #[test]
    fn format() {
        let normal_score = Score::new(Rank::SEVEN);
        assert_eq!(format!("{}", normal_score), "7");

        let blackjack = Score::new(Rank::TEN) + Rank::FIVE + Rank::SIX;
        assert_eq!(format!("{}", blackjack), "21 Blackjack!");

        let busted = Score::new(Rank::TEN) + Rank::FOUR + Rank::EIGHT;
        assert_eq!(format!("{}", busted), "22 busted!");
    }

    #[test]
    fn format_with_ace() {
        let single_ace = Score::new(Rank::ACE);
        assert_eq!(format!("{}", single_ace), "1 or 11");

        let blackjack = Score::new(Rank::ACE) + Rank::TEN;
        assert_eq!(format!("{}", blackjack), "11 or 21 Blackjack!");

        let ace_is_just_one = Score::new(Rank::ACE) + Rank::NINE + Rank::EIGHT;
        // special score is busted so not be showed
        assert_eq!(format!("{}", ace_is_just_one), "18");

        let busted = Score::new(Rank::ACE) + Rank::EIGHT + Rank::EIGHT + Rank::SEVEN;
        assert_eq!(format!("{}", busted), "24 busted!");
    }
}

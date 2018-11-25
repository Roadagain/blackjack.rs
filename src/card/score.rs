#![cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]

use super::Rank;
use std::convert::From;
use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Score {
    value: u32,
    has_ace: bool,
}

impl Score {
    fn new(value: u32, has_ace: bool) -> Self {
        Self { value, has_ace }
    }

    fn ace_as_eleven(&self) -> Self {
        if self.has_ace {
            Self::new(self.value + 10, false)
        } else {
            Self::new(self.value, false)
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

impl From<Rank> for Score {
    fn from(rank: Rank) -> Self {
        let value = match rank {
            Rank::JACK | Rank::QUEEN | Rank::KING => 10,
            n => n as u32,
        };
        Self::new(value, rank == Rank::ACE)
    }
}

impl Add<Score> for Score {
    type Output = Score;

    fn add(self, rank: Score) -> Self {
        Self::new(self.value + rank.value, self.has_ace || rank.has_ace)
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
        let jack_score = Score::from(Rank::JACK);
        assert_eq!(jack_score.value, 10);

        let queen_score = Score::from(Rank::QUEEN);
        assert_eq!(queen_score.value, 10);

        let king_score = Score::from(Rank::KING);
        assert_eq!(king_score.value, 10);
    }

    #[test]
    fn add_score() {
        let score = Score::from(Rank::SIX);
        let result = score + Score::from(Rank::SEVEN);
        assert_eq!(result.value, 13);
    }

    #[test]
    fn bust_score() {
        let single_bust = Score::new(30, false);
        assert!(single_bust.is_busted());

        let blackjack_with_ace = Score::new(21, true);
        // special score (A is 11) busts for 31, but it must be not busted
        assert!(!blackjack_with_ace.is_busted());
    }

    #[test]
    fn format() {
        let normal_score = Score::from(Rank::SEVEN);
        assert_eq!(format!("{}", normal_score), "7");

        let blackjack = Score::new(21, false);
        assert_eq!(format!("{}", blackjack), "21 Blackjack!");

        let busted = Score::new(22, false);
        assert_eq!(format!("{}", busted), "22 busted!");
    }

    #[test]
    fn format_with_ace() {
        let single_ace = Score::new(1, true);
        assert_eq!(format!("{}", single_ace), "1 or 11");

        let blackjack = Score::new(11, true);
        assert_eq!(format!("{}", blackjack), "11 or 21 Blackjack!");

        let ace_is_just_one = Score::new(18, true);
        // special score is busted so not be showed
        assert_eq!(format!("{}", ace_is_just_one), "18");

        let busted = Score::new(24, true);
        assert_eq!(format!("{}", busted), "24 busted!");
    }
}

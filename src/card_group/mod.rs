mod deck;
pub use self::deck::Deck;
use card::Card;
use itertools::free::join;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::fmt;

pub struct CardGroup {
    cards: VecDeque<Card>,
}

impl CardGroup {
    pub fn new(cards: VecDeque<Card>) -> Self {
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let (front, back) = self.cards.as_mut_slices();
        rng.shuffle(front);
        rng.shuffle(back);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }
}

impl fmt::Display for CardGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_iter = self.cards.iter();
        write!(f, "[{}]", join(cards_iter, ", "))
    }
}

#[cfg(test)]
mod test {
    use super::CardGroup;
    use card::Card;
    use card::Rank;
    use card::Suit;
    use std::collections::VecDeque;

    #[test]
    fn draw_returns_top() {
        let top = Card::new(Rank::TEN, Suit::CLUB);
        let bottom = Card::new(Rank::TWO, Suit::DIAMOND);
        let cards_vec = vec![top, bottom];
        let cards = VecDeque::from(cards_vec);
        let mut card_group = CardGroup::new(cards);
        let drawed = card_group.draw();
        let expected = Some(Card::new(Rank::TEN, Suit::CLUB));
        assert_eq!(drawed, expected)
    }

    #[test]
    fn draw_from_empty_returns_none() {
        let cards = VecDeque::new();
        let mut card_group = CardGroup::new(cards);
        let drawed = card_group.draw();
        assert_eq!(drawed, None);
    }
}

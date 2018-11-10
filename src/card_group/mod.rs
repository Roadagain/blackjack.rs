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

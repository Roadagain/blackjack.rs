extern crate itertools;
extern crate rand;

use card_group::card_group::CardGroup;
use card_group::deck::rand::{thread_rng, Rng};
use std::fmt;

pub struct Deck {
    pub card_group: CardGroup,
}

impl Deck {
    pub fn new() -> Deck {
        let card_group = CardGroup::new();
        return Deck { card_group: card_group };
    }

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.card_group.cards);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card_group)
    }
}

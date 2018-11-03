extern crate itertools;

use card_group::card_group::CardGroup;
use std::fmt;

pub struct Deck {
    card_group: CardGroup,
}

impl Deck {
    pub fn new() -> Deck {
        let card_group = CardGroup::new();
        let mut instance = Deck { card_group: card_group };
        instance.card_group.shuffle();
        return instance;
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card_group)
    }
}

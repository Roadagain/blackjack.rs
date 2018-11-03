extern crate itertools;

use card_group::card_group::CardGroup;
use std::fmt;

pub struct Deck {
    card_group: CardGroup,
}

impl Deck {
    pub fn new() -> Deck {
        let mut card_group = CardGroup::new();
        card_group.shuffle();
        return Deck {
            card_group: card_group,
        };
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card_group)
    }
}

use super::CardGroup;
use std::fmt;

pub struct Deck {
    card_group: CardGroup,
}

impl Deck {
    pub fn new() -> Deck {
        let mut card_group = CardGroup::all_cards();
        card_group.shuffle();
        Deck { card_group }
    }

    pub fn shuffle(&mut self) {
        self.card_group.shuffle()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card_group)
    }
}

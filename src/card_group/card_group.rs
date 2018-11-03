extern crate itertools;

use card::card::Card;
use card_group::card_group::itertools::free::join;
use std::fmt;

pub struct CardGroup {
    pub cards: Vec<Card>,
}

impl CardGroup {
    pub fn new() -> CardGroup {
        let all_cards = (1..14).map(|x| Card::new(x));
        return CardGroup { cards: all_cards.collect() };
    }
}

impl fmt::Display for CardGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_iter = self.cards.iter();
        write!(f, "[{}]", join(cards_iter, ", "))
    }
}

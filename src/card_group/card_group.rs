extern crate itertools;
extern crate rand;

use card::card::Card;
use card::rank;
use card::suit::Suit;
use card_group::card_group::itertools::free::join;
use card_group::card_group::rand::{thread_rng, Rng};
use std::fmt;

pub struct CardGroup {
    cards: Vec<Card>,
}

impl CardGroup {
    pub fn new(cards: Vec<Card>) -> CardGroup {
        return CardGroup { cards: cards };
    }

    pub fn all_number(suit: Suit) -> CardGroup {
        let range = rank::RANGE;
        let all_cards = range.map(|x| Card::new(x, suit));
        return CardGroup::new(all_cards.collect());
    }

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.cards);
    }
}

impl fmt::Display for CardGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_iter = self.cards.iter();
        write!(f, "[{}]", join(cards_iter, ", "))
    }
}

extern crate itertools;
extern crate rand;

use card::Card;
use deck::itertools::free::join;
use deck::rand::{thread_rng, Rng};
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let all_cards = (1..14).map(|x| Card::new(x));
        let mut cards: Vec<Card> = all_cards.collect();
        thread_rng().shuffle(&mut cards);
        return Deck { cards: cards };
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_iter = self.cards.iter();
        write!(f, "[{}]", join(cards_iter, ", "))
    }
}

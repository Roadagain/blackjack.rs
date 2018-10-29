extern crate rand;

use card::Card;
use deck::rand::{thread_rng, Rng};
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = (1..14).map(|x| Card::new(x)).collect();
        thread_rng().shuffle(&mut cards);
        return Deck { cards: cards };
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_str: Vec<String> = self.cards.iter().map(|c| format!("{}", c)).collect();
        let joined = cards_str.join(", ");
        write!(f, "[{}]", joined)
    }
}

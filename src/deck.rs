extern crate rand;

use card::Card;
use deck::rand::{thread_rng, Rng};

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

extern crate rand;

use deck::rand::{thread_rng, Rng};

pub struct Deck {
    pub cards: Vec<i32>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<i32> = (1..14).collect();
        thread_rng().shuffle(&mut cards);
        return Deck { cards: cards };
    }
}

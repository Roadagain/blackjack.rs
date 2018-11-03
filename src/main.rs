extern crate blackjack;

use blackjack::card_group::Deck;

fn main() {
    let deck = Deck::new();
    println!("{}", deck);
}

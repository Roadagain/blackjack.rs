extern crate blackjack;

use blackjack::card_group::Deck;

fn main() {
    let mut deck = Deck::new();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}

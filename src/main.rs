pub mod card;
pub mod deck;

use deck::Deck;

fn main() {
    let deck = Deck::new();
    println!("{}", deck);
}

pub mod card;
pub mod card_group;

use card_group::deck::Deck;

fn main() {
    let deck = Deck::new();
    println!("{}", deck);
}

mod deck;
pub use self::deck::Deck;
use card::Card;
use card::Rank;
use card::Suit;
use itertools::free::join;
use rand::{thread_rng, Rng};
use std::fmt;

pub struct CardGroup {
    cards: Vec<Card>,
}

impl CardGroup {
    pub fn new(cards: Vec<Card>) -> CardGroup {
        CardGroup { cards }
    }

    pub fn all_number(suit: Suit) -> CardGroup {
        let range = Rank::RANGE;
        let all_cards = range.map(|x| Card::new(x, suit));
        CardGroup::new(all_cards.collect())
    }

    pub fn all_cards() -> CardGroup {
        let mut all_cards: Vec<Card> = Vec::new();
        for suit in Suit::iterator() {
            let range = Rank::RANGE;
            let all_numbers = range.map(|x| Card::new(x, *suit));
            all_cards.extend(all_numbers);
        }
        CardGroup::new(all_cards)
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

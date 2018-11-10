use super::CardGroup;
use card::Card;
use card::Rank;
use card::Suit;
use std::collections::VecDeque;
use std::fmt;

pub struct Deck {
    card_group: CardGroup,
}

impl Deck {
    pub fn new() -> Self {
        let mut all_cards: VecDeque<Card> = VecDeque::new();
        for suit in Suit::iter() {
            let iter = Rank::into_iter();
            let all_numbers = iter.map(|x| Card::new(x, suit));
            all_cards.extend(all_numbers);
        }

        Self {
            card_group: CardGroup::new(all_cards),
        }
    }

    pub fn shuffle(&mut self) {
        self.card_group.shuffle()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.card_group.draw()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card_group)
    }
}

use rand::rng;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::card::Suit;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
        
        for suit in suits {
            for value in 1..=13 {
                cards.push(Card { value, suit });
            }
        }
        
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        let mut dealt = Vec::new();
        for _ in 0..num_cards {
            if let Some(card) = self.cards.pop() {
                dealt.push(card);
            }
        }
        dealt
    }
}
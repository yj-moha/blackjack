#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub value: u8,
    pub suit: Suit
}

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Club, 
    Diamond, 
    Heart, 
    Spade
}
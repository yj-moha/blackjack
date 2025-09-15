use crate::card::Card;

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name, hand: Vec::new() }
    }
}
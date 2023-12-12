use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
    copies: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card: id: {}, winning_numbers: {:?}, card_numbers: {:?}",
            self.id, self.winning_numbers, self.card_numbers
        )?;
        write!(f, "")
    }
}

impl Card {
    pub fn new(id: i32, winning_numbers: Vec<i32>, card_numbers: Vec<i32>, copies: i32) -> Card {
        Card{
            id,
            winning_numbers,
            card_numbers,
            copies,
        }
    }
    pub fn id(self) -> i32 {
        return self.id
    }

    pub fn winning_numbers(&self) -> Vec<i32> {
        return self.winning_numbers
    }

    pub fn card_numbers(self) -> Vec<i32> {
        return self.card_numbers
    }

    pub fn copies(self) -> i32 {
        return self.copies
    }
}

pub struct CardList {
    cards: Vec<Card>
}

impl CardList {
    pub fn new(cards: Vec<Card>) -> CardList {
        CardList{
            cards,
        }
    }
    pub fn increment_winner_copies() -> () {

    }
}
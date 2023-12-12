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
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn winning_numbers(&self) -> &Vec<i32> {
        &self.winning_numbers
    }

    pub fn card_numbers(&self) -> &Vec<i32> {
        &self.card_numbers
    }

    pub fn copies(&self) -> &i32 {
        &self.copies
    }

    pub fn inc_copies(&mut self) {
        self.copies+=1
    }

    pub fn winner_ct(&self) -> usize {
        self.card_numbers()
            .iter()
            .filter(|c| self.winning_numbers().iter().find(|c1| c == c1).is_some())
            .count()
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

    pub fn get(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn increment_winner_copies(&mut self, idx: usize) {
        let winner_ct = self.cards.get(idx).unwrap().winner_ct();
        for i in idx+1..=idx+winner_ct {
            let mut new_card = self.cards.get(i).unwrap().clone();
            new_card.inc_copies();
            self.cards[i] = new_card;
            println!("source: {} - range: {:?} - incrementing copies for card : {}", idx, idx+1..idx+winner_ct, i);
            self.increment_winner_copies(i);
        }
    }
}
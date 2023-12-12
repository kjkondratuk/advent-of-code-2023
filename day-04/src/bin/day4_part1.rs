use std::fmt;
use std::fmt::Formatter;
use helpers::lines;
use std::string::ParseError;

fn main() {
    let input = include_str!("input.txt");
    let lines = lines(input);
    let parse_result = parse(lines);
    let v = match parse_result {
        Ok(r) => process(r),
        Err(err) => {
            println!("there was an error: {:?}", err);
            0
        }
    };
    println!("{}", v)
}

#[derive(Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
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

fn parse(lines: Vec<&str>) -> Result<Vec<Card>, ParseError> {
    let mut games = vec![];
    for l in lines {
        let rec_parts = l
            .split(": ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let game_id = rec_parts
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
            .get(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let data = rec_parts
            .get(1)
            .unwrap()
            .split(" | ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let winning_numbers = data
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|s| String::from(s))
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let card_numbers = data
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|s| String::from(s))
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // println!(
        //     "{:?}, winning_numbers: {:?}, card_numbers: {:?}",
        //     game_id, winning_numbers, card_numbers
        // );
        games.push(Card {
            id: game_id,
            winning_numbers,
            card_numbers,
        })
    }

    Ok(games)
}

fn process(data: Vec<Card>) -> i32 {
    let mut acc = 0;

    // Make a COPY of ALL OUR FUCKING DATA because the rust compiler is a grumpy old man.
    for card in data.clone().into_iter() {
        // println!("incrementing card counter: {}", card_count);
        let mut score = 0;
        let winner_ct = card.card_numbers
            .iter()
            .filter(|c| card.winning_numbers.iter().find(|c1| c == c1).is_some())
            .count();

            if winner_ct == 1 {
                // println!("adding points: {}", 1);
                score = score + 1;
            }
            if winner_ct > 1 {
                // println!("adding points: {}", 2_i32.pow((winner_ct - 1) as u32));
                score = 2_i32.pow((winner_ct - 1) as u32);
            }

            acc = acc + score;
    }

    acc
}

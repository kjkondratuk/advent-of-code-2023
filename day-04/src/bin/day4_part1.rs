use std::fmt;
use std::fmt::Formatter;
use helpers::lines;
use std::string::ParseError;

fn main() {
    let input = include_str!("part_1.txt");
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
    copies: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card: id: {}, winning_numbers: {:?}, card_numbers: {:?}",
            self.id, self.winning_numbers, self.card_numbers
        )?;
        // for d in &self.data {
        //     write!(f, "{}", d)?;
        // }
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
            copies: 1,
        })
    }

    Ok(games)
}

fn process(mut data: Vec<Card>) -> i32 {
    let mut idx = 0;
    // let mut card_count = 0;

    // Make a COPY of ALL OUR FUCKING DATA because the rust compiler is a grumpy old man.
    for card in data.clone().into_iter() {
        // println!("incrementing card counter: {}", card_count);
        // card_count += 1;
        // let mut score = 0;
        let winner_ct = card.card_numbers
            .iter()
            .filter(|c| card.winning_numbers.iter().find(|c1| c == c1).is_some())
            .count();
        // println!("{}", card);
        println!("card: {}, winner_ct: {}", card.id, winner_ct);

        // card_count += winner_ct;

        // Increment copies
        let mut done = false;
        while !done {
            increment_impacted(&mut data, idx, 0, &mut done);
        }
        // for upcoming in idx+1..idx+1+winner_ct {
        //     let mut up_card = data[upcoming].clone();
        //     up_card.copies = up_card.copies + data[upcoming].copies;
        //     // println!("applying card: {}, mulitplier: {}", up_card.id, up_card.copies);
        //     // card_count += up_card.copies;
        //     data[upcoming] = up_card;
        //
        //     // println!("game: {}, applying copies: {}", data[upcoming].id, data[upcoming].copies);
        // }

        // if winner_ct >= 1 {
        //     println!("adding points: {}", 1);
        //     score = score + 1;
        // }
        // if winner_ct > 1 {
        //     println!("adding points: {}", 2_i32.pow((winner_ct - 1) as u32));
        //     score = 2_i32.pow((winner_ct - 1) as u32);
        // }

        // have to reference the fucking mutable one instead of the copy we're looping over...  Rust is dumb sometimes...
        // acc = acc + score * data[idx].copies;
        idx += 1;
    }

    let mut acc:i32 = 0;
    for card in data {
        println!("card: {}, copies: {}", card.id, card.copies);
        acc = acc + 1 + card.copies;
    }
    acc
    // card_count
    // 0
}

fn increment_impacted(mut data: &mut Vec<Card>, start_idx: usize, mut index: usize, mut done: bool) -> () {
    let c = data.get_mut(start_idx + index);
    match c {
        Some(card) => {
            let winner_ct = card.card_numbers
                .iter()
                .filter(|c| card.winning_numbers.iter().find(|c1| c == c1).is_some())
                .count();
            card.copies += 1;
            //     up_card.copies = up_card.copies + data[upcoming].copies;
            //     // println!("applying card: {}, mulitplier: {}", up_card.id, up_card.copies);
            //     // card_count += up_card.copies;
            //     data[upcoming] = up_card;
            index += 1;
        }
        None => {
            done = true;
        }
    }

}
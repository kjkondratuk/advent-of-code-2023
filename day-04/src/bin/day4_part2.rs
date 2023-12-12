use helpers::lines;
use std::string::ParseError;
use card_list::Card;
use crate::card_list::CardList;

mod card_list;

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
        games.push(Card::new(
            game_id,
            winning_numbers,
            card_numbers,
            1,
        ))
    }

    Ok(games)
}

fn process(data: Vec<Card>) -> i32 {
    let acc = 0;
    let data_clone = data.clone();

    let _cl = CardList::new(data);

    // Make a COPY of ALL OUR FUCKING DATA because the rust compiler is a grumpy old man.
    for (_idx, card) in data_clone.iter().enumerate() {
        let _winner_ct = card.card_numbers()
            .iter()
            .filter(|c| card.winning_numbers().iter().find(|c1| c == c1).is_some())
            .count();

        // increment_winner_copies(idx, data.clone().as_mut(), winner_ct);
    }

    acc
}

// fn increment_winner_copies(idx: usize, l: usize, cards: &mut Vec<Card>, next_x: usize) -> &mut Vec<Card> {
//     let mut loop_ct = 0;
//     for c in &mut cards[idx+1..idx+next_x] {
//         let mut new_c = c.clone();
//         new_c.copies += 1;
//         *c = new_c;
//         if idx + next_x < l {
//             // let winning_nbr = c.winning_numbers.clone();
//             // let winner_ct = c.card_numbers
//             //     .iter()
//             //     .filter(|c| c.winning_numbers.iter().find(|c1| c == c1).is_some())
//             //     .count();
//
//             loop_ct += 1;
//             increment_winner_copies(idx+loop_ct, l, cards.clone().as_mut(), 0/*winner_ct*/);
//         }
//     }
//     cards
// }

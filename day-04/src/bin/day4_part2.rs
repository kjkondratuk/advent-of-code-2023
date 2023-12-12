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
    let mut cc = 0;
    let mut acc = 0;
    let data_clone = data.clone();

    let mut cl = CardList::new(data);

    // Make a COPY of ALL OUR FUCKING DATA because the rust compiler is a grumpy old man.
    for (idx, card) in data_clone.iter().enumerate() {
        let mut score = 0;
        let winner_ct = card.winner_ct();

        cl.increment_winner_copies(idx);
        // increment_winner_copies(idx, data.clone().as_mut(), winner_ct);

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

    for c in cl.get().iter() {
        println!("{} has {} copies", c.id(), c.copies());
        cc += c.copies();
    }

    // acc
    cc
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

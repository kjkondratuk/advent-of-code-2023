extern crate helpers;

use crate::ParseError::{InvalidGameData, InvalidPrefixBlock};
use helpers::lines;

struct Game {
    id: i32,
    set: Vec<Pull>,
}

struct Pull {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn new(id: i32) -> Game {
        Self { id, set: vec![] }
    }
}

// impl fmt::Display for Game {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "Game {{ id: {}, set: {{", self.id)?;
//
//         for (key, value) in &self.set {
//             write!(f, " {}: {},", key, value)?;
//         }
//
//         write!(f, " }} }}")
//     }
// }

fn main() {
    let input = include_str!("input.txt");
    let lines = lines(input);
    let parse_result = parse(lines);
    if !parse_result.is_ok() {
        eprintln!(
            "an error occurred parsing the data: {:?}",
            parse_result.as_ref().err()
        )
    }

    let _ = process(parse_result.unwrap());
}

#[derive(Debug)]
enum ParseError {
    InvalidPrefixBlock,
    InvalidGameData,
}

fn parse(lines: Vec<&str>) -> Result<Vec<Game>, ParseError> {
    let games = Vec::new();
    for l in lines {
        let rec_parts: Vec<String> = l.split(":").map(|v| String::from(v)).collect();

        if rec_parts.len() == 2 {
            let game_int = rec_parts
                .get(0)
                .unwrap()
                .split(" ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
                .get(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            println!("game_int: {}", game_int);

            let sets = rec_parts
                .get(1)
                .unwrap()
                .split("; ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
                .iter()
                .map(|set| {
                    let pull = set
                        .split(", ")
                        .map(|s| String::from(s))
                        .collect()
                        .

                });
        } else {
            return Err(InvalidPrefixBlock);
        }
    }

    Ok(games)
}

fn process(games: Vec<Game>) -> () {}

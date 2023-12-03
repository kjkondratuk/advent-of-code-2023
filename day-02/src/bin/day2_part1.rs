extern crate helpers;

use std::collections::HashMap;
use helpers::lines;
use crate::ParseError::{InvalidPrefixBlock};

struct Game<'a> {
    id: i32,
    set: &'a HashMap<String, i32>,
}

impl<'a> Game<'a> {
    fn new(id: i32) -> Game<'a> {
        Self {
            id,
            set: &HashMap::new(),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines = lines(input);
    let parse_result = parse(lines);
    if !parse_result.is_ok() {
        eprintln!("an error occurred parsing the data: {:?}", parse_result.err())
    }

    let result = process(parse_result.unwrap());
}

#[derive(Debug)]
enum ParseError {
    InvalidPrefixBlock(Vec<String>),
}

fn parse(lines: Vec<&str>) -> Result<Vec<Game>, ParseError> {
    let games = Vec::new();
    for l in lines {
        let mut game;
        let prefix_parts: Vec<String> = l
            .split(":")
            .map(|v| String::from(v))
            .collect();
        if prefix_parts.len() == 2 {
            let game_prefix_parts: Vec<String> = prefix_parts.get(0).unwrap().split(" ")
                .map(|v| String::from(v))
                .collect();
            if game_prefix_parts.len() == 2 {
                let result = *game_prefix_parts.get(1).parse();
                game = Game::new(result);
                // TODO : fix types here
            } else {
                return Err(InvalidPrefixBlock(game_prefix_parts))
            }

            let draws_parts: Vec<String> = prefix_parts.get(1).unwrap().split(";").collect();
            for draw in draws_parts {
                let draws = draw.split(",").collect();
                // TODO : left off here

            }
        } else {
            return Err(InvalidPrefixBlock(prefix_parts))
        }
    }

    Ok(games)
}

fn process(games: Vec<Game>) -> () {

}

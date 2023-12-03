extern crate helpers;

use std::collections::HashMap;
use helpers::lines;

struct Game {
    id: i32,
    set: HashMap<str, i32>,
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

}

fn parse(lines: Vec<&str>) -> Result<Vec<Game>, ParseError> {
    let games = Vec::new();
    for l in lines {

    }

    Ok(games)
}

fn process(games: Vec<Game>) -> () {

}

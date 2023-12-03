extern crate helpers;

use crate::ParseError::{InvalidGameData, InvalidPrefixBlock};
use helpers::lines;
use std::fmt;
use std::fmt::{Formatter};

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

    let result = process(parse_result.unwrap());
    println!("result: {}", result)
}

fn process(games: Vec<Game>) -> i32 {
    let flagged_games = games
        .iter()
        .map(|g| {
            (
                g.id,
                g.set
                    .iter()
                    .any(|p| p.red > 12 || p.green > 13 || p.blue > 14),
            )
        })
        .collect::<Vec<(i32, bool)>>();

    let mut acc = 0;
    for (i, invalid) in flagged_games {
        if invalid {
            match games.get((i - 1) as usize) {
                Some(v) => {
                    // let violations = v.set.iter().map(|p| {
                    //     if p.red > 12 {
                    //         format!("red: {}", p.red)
                    //     } else if p.green > 13 {
                    //         format!("green: {}", p.green)
                    //     } else if p.blue > 14 {
                    //         format!("blue: {}", p.blue)
                    //     } else {
                    //         format!("")
                    //     }
                    // }).filter(|p| p != "").collect::<Vec<String>>();
                    // println!("invalid game: {}, pulls: {}, violations: {:?}", i, v.set.len(), violations);
                    acc += v.id;
                },
                None => println!("invalid game: {}, game: <none>", i),
            }
        }
    }

    games.iter().map(|g| g.id).sum::<i32>() - acc
}

#[derive(Debug)]
enum ParseError {
    InvalidPrefixBlock,
    InvalidGameData,
}

fn parse(lines: Vec<&str>) -> Result<Vec<Game>, ParseError> {
    let mut games = Vec::new();
    for l in lines {
        let rec_parts: Vec<String> = l.split(":").map(|v| String::from(v)).collect();

        if rec_parts.len() == 2 {
            // extract the game id from the record
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
            // println!("game_int: {}", game_int);

            let mut game = Game::new(game_int);

            // extract the pulls from the record
            let sets = rec_parts
                .get(1)
                .unwrap()
                .split("; ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>();

            for s in sets {
                let mut pull = Pull {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                let pairs = s
                    .split(", ")
                    .map(|s| String::from(s.trim()))
                    .collect::<Vec<String>>();

                // println!("set: {}", s);
                for p in pairs {
                    // println!("pair: {}", p);
                    let parts = p
                        .split(" ")
                        .map(|s| String::from(s))
                        .collect::<Vec<String>>();
                    match (parts.get(0), parts.get(1)) {
                        (Some(nbr), Some(label)) => {
                            if label == "red" {
                                pull.red = nbr.parse().unwrap()
                            } else if label == "green" {
                                pull.green = nbr.parse().unwrap()
                            } else if label == "blue" {
                                pull.blue = nbr.parse().unwrap()
                            } else {
                                return Err(InvalidGameData);
                            }
                        }
                        _ => return Err(InvalidGameData),
                    }
                }

                // println!("{:?}", pull);
                game.set.push(pull);
            }

            games.push(game)
        } else {
            return Err(InvalidPrefixBlock);
        }
    }

    Ok(games)
}

struct Game {
    id: i32,
    set: Vec<Pull>,
}

#[derive(Debug)]
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

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Game {{ id: {}, set: {{", self.id)?;

        for (i, pull) in self.set.iter().enumerate() {
            write!(
                f,
                " s: {} {{ red: {}, green: {}, blue: {} }}",
                i, pull.red, pull.green, pull.blue
            )?;
        }

        write!(f, " }} }}")
    }
}
extern crate helpers;

use std::num::ParseIntError;
use helpers::lines;

fn main() {
    let input = include_str!("input.txt");
    let lines = lines(input);

    let result = process(lines);
    match result {
        Ok(result) => {
            println!("Result: {}", result)
        }
        Err(err) => {
            eprintln!("a processing error occurred: {}", err.to_string())
        }
    }
}

fn process(input: Vec<&str>) -> Result<i32, ParseIntError> {
    let mut result = Vec::new();

    // iterate over input
    for l in input {
        // println!("processing input: {}", l);
        let mut first= -1i32;
        let mut last= -1i32;

        // tokenize characters and capture if numeric
        for (_, c) in l.char_indices() {
            if c.is_numeric() {
                if first == -1 {
                    match c.to_digit(10) {
                        Some(digit) => {
                            first = digit as i32
                        }
                        None => {
                            panic!("{} is not a digit!", c)
                        }
                    };
                    // println!("setting first to {} - {}", c, first)
                } else {
                    match c.to_digit(10) {
                        Some(digit) => {
                            last = digit as i32
                        }
                        None => {
                            panic!("{} is not a digit!", c)
                        }
                    };
                    // println!("setting last to {} - {}", c, last)
                }
            }
        }

        // parse numeric from string
        let num: Result<i32, _>;
        if first == -1 && last == -1 {
            num = Ok(0) // if there are no numbers, treat as 0
        } else if last == -1 {
            num = format!("{}{}", first, first).parse() // treat one first value as first and last
        } else {
            num = format!("{}{}", first ,last).parse()
        }

        match num {
            Ok(num) => {
                result.push(num);
            }
            Err(err) => {
                return Err(err)
            }
        }

    }

    let mut acc = 0;
    for r in result {
        acc += r
    }
    Ok(acc)
}

extern crate helpers;

use helpers::lines;
use std::num::ParseIntError;

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
    let mut acc = 0;
    let mut token_set = (0..10).map(|n| n.to_string()).collect::<Vec<String>>();
    token_set.
        extend(vec!(
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string())
        );

    // iterate over input lines
    for l in input {
        let mut first_idx: i32 = -1;
        let mut last_idx: i32 = -1;
        let mut first = "";
        let mut last = "";
        // check find index of first and last
        for v in &token_set {
            // println!("looking for {} in {}", v.to_string(), l);
            match l.find(v.as_str()) {
                Some(idx) => {
                    if first_idx == -1 || idx < first_idx as usize {
                        first_idx = idx as i32;
                        first = v;
                        // println!("found first {} at {}", v, first);
                    }
                }
                _ => {}
            }
        }

        // NOTE : it is SUPER critical to reverse here
        let rev: String = l.chars().rev().collect();
        for v in &token_set {
            // println!("looking for {} in {}", v.to_string(), rev);
            let rev_token: String = v.chars().rev().collect();
            match rev.find(&rev_token.to_string()) {
                Some(idx) => {
                    if last_idx == -1 || idx < last_idx as usize {
                        last_idx = (idx + rev_token.len() - 1) as i32; // NOTE : NEED to ensure we're referencing the start of the token still (since we reversed both the token and the string)
                        last = v;
                        // println!("found last {} at {}", v, last);
                    }
                }
                _ => {}
            }
        }

        match (first_idx, last_idx) {
            (-1, -1) => {
                // do nothing because we didn't find a number
            }
            (_, _) => {
                // println!("line: {} - first: {} - last: {}", l, first, last);

                if !first.parse::<i32>().is_ok() {
                    first = match first {
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        v => panic!("invalid number expression: {}", v),
                    };
                }

                if !last.parse::<i32>().is_ok() {
                    last = match last {
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        v => panic!("invalid number expression: {}", v),
                    };
                }

                let concat = format!("{}{}", first, last);
                let nbr_result: Result<i32, _> = concat.parse();
                match nbr_result {
                    Ok(nbr) => {
                        // println!("adding: {}", nbr);
                        acc += nbr
                    }
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
        }
    }

    Ok(acc)
}

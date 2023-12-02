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

    // iterate over input lines
    for l in input {
        let mut first_idx: i32 = -1;
        let mut last_idx: i32 = -1;
        let mut first: i32 = -1;
        let mut last: i32 = -1;
        // check find index of first and last
        for v in 0..10 {
            // println!("looking for {} in {}", v.to_string(), l);
            match l.find(&v.to_string()) {
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

        let rev: String = l.chars().rev().collect();
        for v in 0..10 {
            // println!("looking for {} in {}", v.to_string(), rev);
            match rev.find(&v.to_string()) {
                Some(idx) => {
                    if last_idx == -1 || idx < last_idx as usize {
                        last_idx = (idx) as i32;
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

                let concat = format!("{}{}", first, last);
                let nbr_result: Result<i32, _> = concat.parse();
                match nbr_result {
                    Ok(nbr) => {
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

extern crate helpers;

use std::num::ParseIntError;
use helpers::lines;

fn main() {
    let input = include_str!("alt_input.txt");
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

fn process(input: Vec<&str>) -> Result<u64, ParseIntError> {
    let mut result = Vec::new();

    // iterate over input
    for l in input {
        // println!("processing input: {}", l);

        let replace_pairs = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];

        let mut actual = String::from(l);

        while replace_pairs.iter().any(|&word| actual.contains(word.0)) {
            let mut earliest = actual.len();
            let mut len = 0;
            let mut replacement = "";

            for (old, new) in replace_pairs {
                match actual.find(old) {
                    Some(idx) => {
                        if idx < earliest {
                            // println!("earliest: {}, len: {}, replacement: {}", idx, old.len(), new);
                            earliest = idx;
                            len = old.len();
                            replacement = new;
                        }
                    }
                    _ => {}
                }
            }

            actual.replace_range(earliest..earliest + len, replacement);
            len = 0;
            earliest = actual.len();
            replacement = "";
        }

        // tokenize characters and capture if numeric
        let mut first= -1i64;
        let mut last= -1i64;

        for (_, c) in actual.char_indices() {
            if c.is_numeric() {
                if first == -1 {
                    match c.to_digit(10) {
                        Some(digit) => {
                            // println!("setting first to {} - {}", c, first);
                            first = digit as i64
                        }
                        None => {
                            panic!("{} is not a digit!", c)
                        }
                    };
                } else {
                    match c.to_digit(10) {
                        Some(digit) => {
                            // println!("setting last to {} - {}", c, last);
                            last = digit as i64
                        }
                        None => {
                            panic!("{} is not a digit!", c)
                        }
                    };
                }
            }
        }

        // parse numeric from string
        let num: Result<u64, _>;
        if first == -1 && last == -1 {
            num = Ok(0) // if there are no numbers, treat as 0
        } else if last == -1 {
            num = format!("{}{}", first, first).parse() // treat one first value as first and last
        } else {
            num = format!("{}{}", first ,last).parse()
        }

        // if String::from(l) != actual {
            println!("l: {}, actual: {} -> {:?}", l, actual, num);
        // }

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

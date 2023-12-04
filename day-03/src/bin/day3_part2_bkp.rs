use std::collections::HashMap;
use helpers::lines;
use regex::Regex;
use std::fmt;
use std::fmt::{format, Formatter};
use std::hash::Hash;
use std::ops::Range;

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

#[derive(Debug)]
enum ParsingError {}

fn parse(lines: Vec<&str>) -> Result<Vec<Block>, ParsingError> {
    let number_pattern: Regex = Regex::new(r"\d+").unwrap();

    let mut blocks: Vec<Block> = vec![];
    let mut prev_line = "".to_string();
    // let mut next_line = "".to_string();
    for (i, l) in lines.iter().map(|&s| String::from(s)).enumerate() {
        // TODO : working on fixing this
        let next = lines.get(i + 1);
        let next_line = match next {
            Some(n) => String::from(n.to_string()),
            None => "".to_string(),
        };
        for cap in number_pattern.captures_iter(l.as_str()) {
            let value = cap.get(0).unwrap();
            let start = value.start();
            let len = value.len();
            let mut local_coord;

            // println!("setting block range: start: {}, len: {}", start, len);
            let block_range: Range<usize>;
            if start == 0 {
                // no padding on left side, because we're at the start of the string
                block_range = start..start + len + 1;
            } else if start + len + 1 > lines.get(i).unwrap().to_string().len() {
                // no padding on the right side beause we're at the end of the string
                block_range = start - 1..start + len;
            } else {
                // regular padding around the value
                block_range = start - 1..start + len + 1;
            }

            let curr_block =
                String::from(&lines.get(i).unwrap().to_string().as_str()[block_range.clone()]);
            let mut prev_block = "".to_string();
            if prev_line.len() > 0 {
                prev_block = String::from(&prev_line.as_str()[block_range.clone()]);
            }
            let mut next_block = "".to_string();
            if next_line.len() > 0 {
                next_block = String::from(&next_line.as_str()[block_range.clone()]);
            }

            if prev_line.len() == 0  {
                local_coord = (0, start - block_range.start)
            } else {
                local_coord = (1, start - block_range.start)
            }

            let nbr = String::from(&lines.get(i).unwrap().to_string().as_str()[start..start + len])
                .parse::<i32>()
                .unwrap();

            let mut data;
            if prev_block.len() == 0 {
                data = vec![curr_block, next_block]
            } else if next_block.len() == 0 {
                data = vec![prev_block, curr_block]
            } else {
                data = vec![prev_block, curr_block, next_block]
            }

            let b = Block {
                coord: (i, start),
                local: local_coord,
                number: nbr,
                start,
                len,
                data,
            };

            // println!("line: {}, number: {}, start: {}, len: {}, has_symbol: {}, data: {:?}", i, b.number, b.start, b.len, b.contains_symbol(), b.data);
            blocks.push(b);
        }

        // push next line to previous
        prev_line = l;
    }
    Ok(blocks)
}

fn process(blocks: Vec<Block>) -> i32 {
    let mut counts: HashMap<String, i32> = HashMap::new();
    blocks.iter().for_each(|b| {
        b.asterisk_coords().iter().for_each(|coord_set: &Vec<(usize, usize)>| {
            coord_set.iter().for_each(|coord| {
                // TODO : coordinates are getting incorrect somewhere here
                counts.entry(format!("{},{}", coord.0, coord.1))
                    .and_modify(|v| *v = *v + 1)
                    .or_insert(0);
            });
        });
        return
    });

    for (k, v) in counts {
        println!("{}: {}", k, v);
    }

    0
}

struct Block {
    // coord : the coordinate of the starting position of the number
    coord: (usize, usize),
    // local : the offset from the upper left of the bounding box
    local: (usize, usize),
    // number : the number in the block
    number: i32,
    // start : the start position of the number on the line
    start: usize,
    // len : the length of the number
    len: usize,
    // data : the raw data block the number is contained in, with a bounding box
    data: Vec<String>,
}

impl Block {
    fn contains_symbol(&self) -> bool {
        self.data.iter().any(|d| {
            Regex::new(r"[^.A-Za-z0-9\n]")
                .unwrap()
                .find_iter(d)
                .next()
                .is_some()
        })
    }

    fn asterisk_coords(&self) -> Option<Vec<(usize, usize)>> {
        let mut data_coords: Vec<(usize, usize)> = vec![];
        for (i, d) in self.data.iter().enumerate() {
            for (n, c) in d.chars().enumerate() {
                if c == '*' {
                    data_coords.push((self.coord.0 - self.local.0 + i, self.coord.1 - self.local.1 + n + 1));
                }
            }
        }

        if data_coords.len() > 0 {
            Some(data_coords)
        } else {
            None
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block: number: {}, start: {}, len: {}, chars: ",
            self.number, self.start, self.len
        )?;
        for d in &self.data {
            write!(f, "{}", d)?;
        }
        write!(f, "")
    }
}

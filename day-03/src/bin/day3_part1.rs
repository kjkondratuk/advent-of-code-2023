use helpers::lines;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;
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

            let nbr = String::from(&lines.get(i).unwrap().to_string().as_str()[start..start + len])
                .parse::<i32>()
                .unwrap();

            let b = Block {
                number: nbr,
                start,
                len,
                data: vec![prev_block, curr_block, next_block],
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
    blocks
        .iter()
        .filter(|&b| b.contains_symbol())
        .map(|b| b.number).sum()
}

struct Block {
    number: i32,
    start: usize,
    len: usize,
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

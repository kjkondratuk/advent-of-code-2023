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
    let number_pattern: Regex = Regex::new(r"\*").unwrap();

    let mut blocks: Vec<Block> = vec![];
    let mut prev_line = "".to_string();
    // let mut next_line = "".to_string();
    for (i, l) in lines.iter().map(|&s| String::from(s)).enumerate() {
        let next = lines.get(i + 1);
        let mut next_line = match next {
            Some(n) => String::from(n.to_string()),
            None => "".to_string(),
        };
        let mut curr_line = lines.get(i).unwrap();
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
                String::from(&curr_line.to_string().as_str()[block_range.clone()]);
            let mut prev_block = "".to_string();
            if prev_line.len() > 0 {
                prev_block = String::from(&prev_line.as_str()[block_range.clone()]);
            }
            let mut next_block = "".to_string();
            if next_line.len() > 0 {
                next_block = String::from(&next_line.as_str()[block_range.clone()]);
            }

            let starts_with_number = Regex::new("^[0-9]+").unwrap();
            let ends_with_number = Regex::new("[0-9]+$").unwrap();

            // grow the bounding boxes to include all of numbers we truncated
            let new_blocks: Vec<String> = vec![(prev_block.to_string(), &prev_line), (curr_block.to_string(), &curr_line.to_string()), (next_block.to_string(), &next_line)].iter().map(|(b, l)| {
                let mut result = b.clone();
                // if b == "6.2" {
                //     println!("block: {}", b);
                // }
                // println!("block: {}", b);
                if starts_with_number.is_match(b) {
                    let mut start_char = b.get(0..1).unwrap().to_string();
                    let mut offset = 0;
                    let mut done = false;
                    while !done && starts_with_number.is_match(start_char.as_str()) {
                        // protection for decrementing a usize out of range
                        if offset + 2 > start {
                            done = true;
                            break;
                        }
                        let range = start - 2 - offset..start - 1 - offset;
                        let r = l.get(range);
                        match r {
                            Some(c) => {
                                result = c.to_string() + result.as_str();
                                start_char = c.to_string();
                            },
                            _ => done = true
                        }
                        offset = offset + 1;
                    }
                }

                if ends_with_number.is_match(b) {
                    let mut end_char = b.get(b.len() - 1..b.len()).unwrap().to_string();
                    let mut offset = 0;
                    let mut done = false;
                    while !done && ends_with_number.is_match(end_char.as_str()) {
                        let r = l.get(start + len + 1 + offset..start + len + 2 + offset);
                        match r {
                            Some(c) => {
                                result = result.to_string() + c;
                                end_char = c.to_string();
                            },
                            _ => done = true
                        }
                        offset = offset + 1;
                    }
                }

                return result
            }).collect::<Vec<String>>();

            // println!("block: {}", new_blocks.concat());

            // let nbr = String::from(&lines.get(i).unwrap().to_string().as_str()[start..start + len])
            //     .parse::<i32>()
            //     .unwrap();

            let b = Block {
                number: 0,
                numbers: vec![],
                start,
                len,
                data: new_blocks,
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
    let b = blocks.iter().filter(|b| b.numbers().len() == 2).collect::<Vec<&Block>>();;

    // for x in &b {
    //     println!("data: {}, numbers: {:?}, gear_ratio: {}", x.data.concat(), x.numbers(), x.numbers().get(0).unwrap() * x.numbers().get(1).unwrap());
    // }

    b.iter().map(|b| b.numbers().get(0).unwrap() * b.numbers().get(1).unwrap()).sum()
    // 0
}

struct Block {
    number: i32,
    start: usize,
    len: usize,
    numbers: Vec<i32>,
    data: Vec<String>,
}

impl Block {
    fn numbers(&self) -> Vec<i32> {
        let mut nbr_strs: Vec<i32> = vec![];
        for row in &self.data {
            for mtc in Regex::new(r"[0-9]+").unwrap().find_iter(row.as_str()) {
                nbr_strs.push(mtc.as_str().parse::<i32>().unwrap())
            }
        }

        // let n = self.numbers.clone();
        // return n
        nbr_strs
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block: number: {}, start: {}, len: {}, chars: ",
            self.number, self.start, self.len
        )?;
        // for d in &self.data {
        //     write!(f, "{}", d)?;
        // }
        write!(f, "")
    }
}

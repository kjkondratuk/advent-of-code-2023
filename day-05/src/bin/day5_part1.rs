use day_05::almanac::{Almanac, AlmanacMapping, MappingKey};
use helpers::lines;
use std::collections::HashMap;

fn main() {
    let input = include_str!("part_1.txt");
    let lines = lines(input);
    let parse_result = parse(lines);
    let process_result = process(parse_result);
    // let v = match parse_result {
    //     Ok(r) => process(r),
    //     Err(err) => {
    //         println!("there was an error: {:?}", err);
    //         0
    //     }
    // };
    println!("{}", process_result)
}

fn process(island_almanac: Almanac) -> i32 {
    let _ = island_almanac.get_seeds().iter().filter(|&seed| {
        let mappings = island_almanac
            .get_mappings();
        let in_range = mappings
            .get(&(Almanac::SEED, Almanac::SOIL))
            .unwrap()
            .iter()
            .filter(|&m| m.in_source_range(*seed))
            .collect::<Vec<&AlmanacMapping>>();
        for m in &in_range {
            println!("in range: {:?}", m);
        }
        return in_range.len() > 0
    });
    0
}

fn parse(lines: Vec<&str>) -> Almanac {
    // parse the first line into a Vec<String>
    let seeds = lines
        .get(0)
        .unwrap()
        .split_whitespace()
        .map(|s| String::from(s))
        .map(|s| s.parse::<i64>().unwrap_or(-1))
        .filter(|&n| n != -1)
        .collect::<Vec<i64>>();

    // trim first 2 lines because they're formatted differently
    let mut remaining_lines = lines[2..].iter().map(|&s| s).collect::<Vec<&str>>();

    let mut mapping: HashMap<(MappingKey, MappingKey), Vec<AlmanacMapping>> = HashMap::new();
    let mut line_counter = 0;
    let mut group_counter = 0;
    while remaining_lines.len() > 0 {
        group_counter += 1;
        // parse the first set of mappings (seed -> soil)
        if remaining_lines[0].to_string() == "" {
            remaining_lines = remaining_lines[1..remaining_lines.len()].to_vec();
        }
        let modified_line = remaining_lines[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .replace("-to-", "-");
            let parts = modified_line.split("-")
            .collect::<Vec<&str>>();
        // let split_line = modified_line.split("-");
        // let modified_vec = split_line.collect::<Vec<&str>>();
        line_counter += 1;

        // println!("processing group: {} at line {} - {}", group_counter, line_counter, modified_line);
        // TODO : there is an issue here with referencing modified_vec with dereferencing
        let type_mapping = (
            parts.get(0).unwrap(),
            parts.get(1).unwrap(),
        );

        let mut value_mapping: Vec<AlmanacMapping> = vec![];
        // println!("iterating over mapping range: {} - {}", line_counter, remaining_lines.len());
        for nl in remaining_lines[1..remaining_lines.len()].iter() {
            // println!("reading value line: {} - {}", line_counter, nl);
            if nl.to_string() == "" {
                // println!("!!!!!! newline !!!!!!");
                break;
            }
            let parts = nl
                .split_whitespace()
                .map(|s| s.parse::<i64>())
                .map(|r| r.unwrap())
                .collect::<Vec<i64>>();
            value_mapping.push(AlmanacMapping::new(parts[0], parts[1], parts[2]));
            line_counter += 1;
        }

        mapping.insert(type_mapping.clone(), value_mapping.clone());

        // println!("inserted {} mappings to {} - {}", value_mapping.len(), type_mapping.0, type_mapping.1);

        // eat up the buffer that's been processed
        // println!("reslicing to range: {} - {}", line_counter, remaining_lines.len());
        let rl = remaining_lines[line_counter..remaining_lines.len()].to_vec();
        // println!("lines remaining: {}", rl.len());
        remaining_lines = rl;
        line_counter = 0;
    }

    Almanac::new(seeds, mapping)
}

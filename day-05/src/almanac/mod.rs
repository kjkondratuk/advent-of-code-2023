use std::collections::HashMap;

#[derive(Default)]
pub struct Almanac {
    seeds: Vec<i64>,
    mappings: HashMap<MappingKey, Vec<AlmanacMapping>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MappingKey {
    pub from: String,
    pub to: String,
}

pub static SEED: &str = &"seed";
pub static SOIL: &str = &"soil";
pub static FERTILIZER: &str = &"fertilizer";
pub static WATER: &str = &"water";
pub static LIGHT: &str = &"light";
pub static TEMPERATURE: &str = &"temperature";
pub static HUMIDITY: &str = &"humidity";
pub static LOCATION: &str = &"location";

impl Almanac {
    pub fn new(seeds: Vec<i64>, mappings: HashMap<MappingKey, Vec<AlmanacMapping>>) -> Almanac {
        Almanac{
            seeds,
            mappings,
        }
    }

    pub fn get_seeds(&self) -> Vec<i64> {
        self.seeds.clone()
    }

    pub fn get_mappings(&self) -> HashMap<MappingKey, Vec<AlmanacMapping>> {
        self.mappings.clone()
    }
}

#[derive(Clone, Debug)]
pub struct AlmanacMapping {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

impl AlmanacMapping {
    pub fn new(dest_start: i64, src_start: i64, length: i64) -> AlmanacMapping {
        return AlmanacMapping{
            dest_start,
            src_start,
            length,
        }
    }

    pub fn in_source_range(&self, nbr: i64) -> bool {
        let result = nbr >= self.dest_start && nbr <= self.dest_start+self.length;
        println!("Checking source range: {} to {} - {} - {}", self.dest_start, self.dest_start+self.length, result, nbr);
        result
    }

    pub fn in_dest_range(&self, nbr: i64) -> bool {
        nbr >= self.src_start && nbr <= self.src_start+self.length
    }
}
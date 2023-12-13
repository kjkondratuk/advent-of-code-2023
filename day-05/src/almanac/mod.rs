use std::collections::HashMap;

#[derive(Default)]
pub struct Almanac {
    seeds: Vec<i64>,
    mappings: HashMap<(MappingKey, MappingKey), Vec<AlmanacMapping>>,
}

pub type MappingKey = &'static str;

impl Almanac {
    pub const SEED: &'static MappingKey = &"seed";
    pub const SOIL: &'static MappingKey = &"soil";
    pub const FERTILIZER: &'static MappingKey = &"fertilizer";
    pub const WATER: &'static MappingKey = &"water";
    pub const LIGHT: &'static MappingKey = &"light";
    pub const TEMPERATURE: &'static MappingKey = &"temperature";
    pub const HUMIDITY: &'static MappingKey = &"humidity";
    pub const LOCATION: &'static MappingKey = &"location";
    pub fn new(seeds: Vec<i64>, mappings: HashMap<(MappingKey, MappingKey), Vec<AlmanacMapping>>) -> Almanac {
        Almanac{
            seeds,
            mappings,
        }
    }

    pub fn get_seeds(&self) -> Vec<i64> {
        self.seeds.clone()
    }

    pub fn get_mappings(&self) -> HashMap<(MappingKey, MappingKey), Vec<AlmanacMapping>> {
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
        nbr >= self.dest_start && nbr <= self.dest_start+self.length
    }

    pub fn in_dest_range(&self, nbr: i64) -> bool {
        nbr >= self.src_start && nbr <= self.src_start+self.length
    }
}
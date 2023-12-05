use advent_of_code::find_numbers_64;
use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
enum MapType {
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc
}

impl FromStr for MapType {
    type Err = String;

    fn from_str(input: &str) -> Result<MapType, Self::Err> {
        match input {
            "seed-to-soil" => Ok(MapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapType::SoilToFert),
            "fertilizer-to-water" => Ok(MapType::FertToWater),
            "water-to-light" => Ok(MapType::WaterToLight),
            "light-to-temperature" => Ok(MapType::LightToTemp),
            "temperature-to-humidity" => Ok(MapType::TempToHumid),
            "humidity-to-location" => Ok(MapType::HumidToLoc),
            _ => Err(input.to_string())
        }
    }
}

#[derive(Debug)]
struct MappingRange {
    destination_start: u64,
    source_start: u64,
    length: u64
}

impl MappingRange {
    fn source_end(&self) -> u64 {
       self.source_start + self.length
    }

    fn destination_end(&self) -> u64 {
        self.destination_start + self.length
    }

    fn can_translate(&self, source: u64) -> bool {
        return source >= self.source_start && source <= self.source_end();
    }

    fn translate(&self, source: u64) -> u64 {
        return self.destination_start + source.abs_diff(self.source_start);
    }
}

#[derive(Debug)]
struct Mapping {
    mtype: MapType,
    ranges: Vec<MappingRange>
}

impl Mapping {
    fn default() -> Mapping {
        Mapping{mtype: MapType::SeedToSoil, ranges: vec![]}
    }
}

fn seed_to_location(mappings: &Vec<Mapping>, seed: &u64) -> u64 {
    let mut next = seed.clone();
    // print!("\n");
    for mapping in mappings {
        for range in &mapping.ranges {
            if range.can_translate(next) {
                let temp = next.clone();
                next = range.translate(next);
                // println!("translated {}, to {}, for stage {:?}", temp, next, mapping.mtype);
                break;
            }
            else {
                // println!("stage {:?}", mapping.mtype)
            }
        }
    }

    next
}

// TODO return iterator, not Vec
fn extrapolate_seeds(seeds: Vec<u64>) -> Vec<u64>  {
    let start_seeds = seeds.iter().step_by(2);
    let lengths = seeds.iter().skip(1).step_by(2);

    std::iter::zip(start_seeds, lengths).into_iter();
    todo!("implement iterator that generates a seed for each start_seed -> start_seed + length")



}

pub fn part_one(input: &str) -> Option<u64> {
    let map_type_re = Regex::new(r"^(?<name>[a-z-]+)").unwrap();

    let mut input = input.lines();

    let _seed_line = input.next().unwrap();
    let seeds = find_numbers_64(_seed_line);

    let mut mappings: Vec<Mapping> = vec![];
    let mut current_mapping = Mapping::default();

    let input = input.skip(1);
    let max_index = input.clone().count() - 1;

    for (i, line) in input.enumerate() {
        if line.chars().nth(0).is_some_and(|c| c.is_alphabetic()) {
            let caps = map_type_re.captures(line).unwrap();
            current_mapping.mtype = MapType::from_str(&caps["name"]).unwrap();
            continue;
        }
        if line.chars().nth(0).is_some_and(|c| c.is_numeric()) {
            let nums = find_numbers_64(line);
            let range = MappingRange{
                destination_start: nums[0],
                source_start: nums[1],
                length: nums[2]
            };
            current_mapping.ranges.push(range);
        }

        if line.trim().is_empty() || i == max_index {
            mappings.push(current_mapping);
            current_mapping = Mapping::default();
        }
    }

    let locations = seeds
    .iter()
    .map(|s| seed_to_location(&mappings, s));


    Some(locations.min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map_type_re = Regex::new(r"^(?<name>[a-z-]+)").unwrap();

    let mut input = input.lines();

    let _seed_line = input.next().unwrap();
    let seeds = find_numbers_64(_seed_line);

    let mut mappings: Vec<Mapping> = vec![];
    let mut current_mapping = Mapping::default();

    let input = input.skip(1);
    let max_index = input.clone().count() - 1;

    for (i, line) in input.enumerate() {
        if line.chars().nth(0).is_some_and(|c| c.is_alphabetic()) {
            let caps = map_type_re.captures(line).unwrap();
            current_mapping.mtype = MapType::from_str(&caps["name"]).unwrap();
            continue;
        }
        if line.chars().nth(0).is_some_and(|c| c.is_numeric()) {
            let nums = find_numbers_64(line);
            let range = MappingRange{
                destination_start: nums[0],
                source_start: nums[1],
                length: nums[2]
            };
            current_mapping.ranges.push(range);
        }

        if line.trim().is_empty() || i == max_index {
            mappings.push(current_mapping);
            current_mapping = Mapping::default();
        }
    }

    println!("here");
    let seeds = extrapolate_seeds(seeds);
    println!("{:?}", seeds.len());
    // let locations = seeds
    // .iter()
    // .map(|s| seed_to_location(&mappings, s));

    // Some(locations.min().unwrap())
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(46));
    }
}

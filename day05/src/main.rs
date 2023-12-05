mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::slice::Iter;
use std::str::FromStr;
use itertools::Itertools;

struct Game {

}

// seeds: 79 14 55 13
//
// seed-to-soil map:
// soil-to-fertilizer map:
// fertilizer-to-water map:
// water-to-light map:
// light-to-temperature map:
// temperature-to-humidity map:
// humidity-to-location map:

struct Mapping {
    destination: u32,
    source: u32,
    length: u32,
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input);

    let sections = input.split("\n\n").collect_vec();
    let mut sections_iter = sections.iter();

    let line0 = sections_iter.next().unwrap();
    let split0 = line0.split(" ").collect_vec();

    let seeds = split0.iter().skip(1).map(|s| u32::from_str(s).unwrap()).collect_vec();

    let seed_to_soil = parse_section(&mut sections_iter);
    let soil_to_fertilizer = parse_section(&mut sections_iter);
    let fertilizer_to_water = parse_section(&mut sections_iter);
    let water_to_light = parse_section(&mut sections_iter);
    let light_to_temperature = parse_section(&mut sections_iter);
    let temperature_to_humidity = parse_section(&mut sections_iter);
    let humidity_to_location = parse_section(&mut sections_iter);

    Ok(Game { })
}

fn parse_section(sections_iter: &mut Iter<&str>) -> Vec<Mapping>{
    let section = sections_iter.next().unwrap().split("\n").collect_vec();
    let mut section_iter = section.iter();
    let name = section_iter.next().unwrap();
    section_iter.take_while(|x| !x.is_empty()).map(|x| {
        let s = x.split(" ").map(|y| u32::from_str(y).unwrap()).collect_vec();
        Mapping {
            destination: s[0],
            source: s[1],
            length: s[2],
        }
    })
        .collect_vec()
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    Ok(0)
}

fn part2(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

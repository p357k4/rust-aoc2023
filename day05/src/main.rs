mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::slice::Iter;
use std::str::FromStr;
use itertools::Itertools;

struct Game {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
    seeds: Vec<u64>,
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
    a: u64,
    b: u64,
    c: u64,
}

fn map(mappings: &Vec<Mapping>, value: u64) -> u64 {
    mappings.iter()
        .find(|&mapping| mapping.b >= value && value >= mapping.a)
        .map(|mapping| mapping.c + (value - mapping.a))
        .unwrap_or(value)
}

#[derive(Clone, Copy)]
struct Range {
    e: u64,
    f: u64,
}

fn split_left(range: Range, a: u64) -> Vec<Range> {
    if range.e < a && a <= range.f {
        vec![Range { e: range.e, f: a - 1 }, Range { e: a, f: range.f }]
    } else {
        vec![range]
    }
}

fn split_right(range: Range, a: u64) -> Vec<Range> {
    if range.e <= a && a < range.f {
        vec![Range { e: range.e, f: a }, Range { e: a + 1, f: range.f }]
    } else {
        vec![range]
    }
}

fn map1(mappings: &Vec<Mapping>, range: Vec<Range>) -> Vec<Range> {
    let splits = mappings.iter()
        .fold(range, |acc, mapping| {
            let split = acc.iter()
                .flat_map(|&r| split_left(r, mapping.a))
                .flat_map(|r| split_right(r, mapping.b))
                .collect_vec();
            split
        });


    let splits = splits.iter()
        .map(|r|{
            mappings.iter()
                .find(|mapping| mapping.a <= r.e && r.f <= mapping.b)
                .map(|mapping| {
                    Range {
                        e: mapping.c + (r.e - mapping.a),
                        f: mapping.c + (r.f - mapping.a),
                    }
                })
                .unwrap_or(*r)
        })
        .collect_vec();

    splits
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

    let seeds = split0.iter().skip(1).map(|s| u64::from_str(s).unwrap()).collect_vec();

    let seed_to_soil = parse_section(&mut sections_iter);
    let soil_to_fertilizer = parse_section(&mut sections_iter);
    let fertilizer_to_water = parse_section(&mut sections_iter);
    let water_to_light = parse_section(&mut sections_iter);
    let light_to_temperature = parse_section(&mut sections_iter);
    let temperature_to_humidity = parse_section(&mut sections_iter);
    let humidity_to_location = parse_section(&mut sections_iter);

    Ok(Game { seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location })
}

fn parse_section(sections_iter: &mut Iter<&str>) -> Vec<Mapping> {
    let section = sections_iter.next().unwrap().split("\n").collect_vec();
    let mut section_iter = section.iter();
    let name = section_iter.next().unwrap();
    section_iter
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let s = x.split(" ").map(|y| u64::from_str(y).unwrap()).collect_vec();
            Mapping {
                a: s[1],
                b: s[1] + s[2] - 1,
                c: s[0],
            }
        })
        .collect_vec()
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let locations = game.seeds.iter().map(|&seed| {
        let soil = map(&game.seed_to_soil, seed);
        let fertilizer = map(&game.soil_to_fertilizer, soil);
        let water = map(&game.fertilizer_to_water, fertilizer);
        let light = map(&game.water_to_light, water);
        let temperature = map(&game.light_to_temperature, light);
        let humidity = map(&game.temperature_to_humidity, temperature);
        let location = map(&game.humidity_to_location, humidity);
        location
    }).collect_vec();

    let result = locations.iter().min().unwrap();
    Ok(*result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;
    let seed_chunks = game.seeds.chunks(2).collect_vec();

    let locations = seed_chunks.iter().map(|&chunk| {
        let seed = vec![Range { e: chunk[0], f: (chunk[0] + chunk[1]) - 1 }];
        let soil = map1(&game.seed_to_soil, seed);
        let fertilizer = map1(&game.soil_to_fertilizer, soil);
        let water = map1(&game.fertilizer_to_water, fertilizer);
        let light = map1(&game.water_to_light, water);
        let temperature = map1(&game.light_to_temperature, light);
        let humidity = map1(&game.temperature_to_humidity, temperature);
        let location = map1(&game.humidity_to_location, humidity);
        let min = location.iter().min_by_key(|range| range.e).unwrap();

        min.e
    }).collect_vec();

    let result = locations.iter().min().unwrap();
    Ok(*result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

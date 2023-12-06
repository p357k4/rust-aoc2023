mod main_test;

use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use itertools::{concat, Itertools};
use nom::bytes::complete::tag;
use nom::character;
use nom::character::complete;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

struct Game {
    races: Vec<Race>,
}

#[derive(Clone, Copy)]
struct Race {
    t: u64,
    d: u64,
}

// Time:      7  15   30
// Distance:  9  40  200

fn parse_time(input: &str) -> IResult<&str, &str> {
    tag("Time:")(input)
}

fn parse_distance(input: &str) -> IResult<&str, &str> {
    tag("Distance:")(input)
}

fn distance(charge_time: u64, race_time: u64) -> u64 {
    let reduced_charge_time = min(charge_time, race_time);
    let distance = (race_time - reduced_charge_time) * reduced_charge_time;
    distance
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect_vec();

    let times = lines[0].iter().flat_map(|s| {
        let result = tuple((parse_time, complete::space1, separated_list0(complete::space1, complete::u64)))(s.as_str());
        let (_, (_, _, times)) = result.unwrap();
        times
    }).collect_vec();
    let distances = lines[1].iter().flat_map(|s| {
        let result = tuple((parse_distance, complete::space1, separated_list0(complete::space1, complete::u64)))(s.as_str());
        let (_, (_, _, distances)) = result.unwrap();
        distances
    }).collect_vec();

    let races = times.iter().zip(distances.iter()).map(|(&t, &d)| Race { t, d }).collect_vec();

    Ok(Game { races })
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let wins = game.races.iter().map(|r| {
        let w = (0..=r.t).map(|t| distance(t, r.t)).filter(|&d| d > r.d).count();
        w as u64
    }).collect_vec();

    let result = wins.iter().fold(1u64, |acc, x| acc * x);
    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let race = game.races.iter().fold(Race { t: 0, d: 0 }, |acc, r| {
        Race {
            t: format!("{}{}", acc.t.to_string(), r.t.to_string()).parse::<u64>().unwrap(),
            d: format!("{}{}", acc.d.to_string(), r.d.to_string()).parse::<u64>().unwrap(),
        }
    });

    let w = (0..=race.t).map(|t| distance(t, race.t)).filter(|&d| d > race.d).count();

    let result = w as u64;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

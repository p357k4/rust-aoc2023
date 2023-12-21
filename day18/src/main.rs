mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use itertools::{Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

#[derive(Clone, Eq, PartialEq)]
struct Plan {
    direction: String,
    length: u32,
    color: String,
}
struct Input {
    dig_plan: Vec<Plan>,
}

fn parse_plan(input: &str) -> IResult<&str, Plan> {
    tuple((alphanumeric1, tag(" "), complete::u32, tag(" (#"), alphanumeric1, tag(")")))(input)
        .map(|(leftover, (direction, _, length, _, color, _))| {
            (leftover, Plan {
                direction: direction.to_string(),
                length,
                color: color.to_string(),
            })
        })
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content);

    let dig_plan = content.split("\n")
        .map(|line| {
            let (leftover, part) = parse_plan(line).unwrap();
            part
        }).collect_vec();

    Ok(Input { dig_plan })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut result = 0;

    Ok(result)
}

fn part2(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;
    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

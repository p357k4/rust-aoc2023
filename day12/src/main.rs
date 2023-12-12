mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;

#[derive(Clone, PartialEq)]
struct Row {
    status: String,
    groups: Vec<u32>,
}

struct Input {
    rows: Vec<Row>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('#'), complete::char('?'), complete::char('.'))))(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(tag(","), complete::u32)(input)
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap()).collect_vec();

    let rows = lines.iter().map(|line| {
        let (leftover, (status, _, groups)) = tuple((parse_string, complete::space1, parse_groups))(line.as_str()).unwrap();
        Row { status: status.iter().join(""), groups }
    }).collect_vec();

    Ok(Input { rows })
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = 0;
    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

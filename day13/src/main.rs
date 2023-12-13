mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;

struct Row {
    notes: Vec<char>,
}

struct Block {
    rows: Vec<Row>,
}

struct Input {
    blocks: Vec<Block>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('#'), complete::char('.'))))(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), complete::u64)(input)
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content);

    let blocks_content = content.split("\n\n").collect_vec();

    let blocks = blocks_content.iter()
        .map(|&block_section| {
            let rows = block_section
                .split("\n")
                .map(|line| {
                    let (leftover, notes) = parse_string(line).unwrap();
                    Row { notes }
                }
                ).collect_vec();
            Block { rows }
        }
        )
        .collect_vec();

    Ok(Input { blocks })
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

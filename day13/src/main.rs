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

struct Block {
    rows: Vec<String>,
}

struct Input {
    blocks: Vec<Block>,
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let blocks_content = content.split("\n\n").collect_vec();

    let blocks = blocks_content.iter()
        .map(|&block_section| {
            let rows = block_section
                .split('\n')
                .map(|v| v.to_string())
                .collect_vec();
            Block { rows }
        }
        )
        .collect_vec();

    Ok(Input { blocks })
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;
    let mut vertical_sum = 0;
    let mut horizontal_sum = 0;

    for block in input.blocks {
        for shift in 1..block.rows.first().map(|v| v.len()).unwrap_or(0) {
            let equal = block.rows.iter().all(|v| v.bytes().take(shift).rev().zip(v.bytes().skip(shift)).all(|(left, right)| left == right));
            if equal {
                vertical_sum += shift;
            }
        }

        for shift in 1..block.rows.len() {
            let equal = block.rows.iter().take(shift).rev().zip(block.rows.iter().skip(shift)).all(|(left, right)| left == right);
            if equal {
                horizontal_sum += shift;
            }
        }
    }

    let result = horizontal_sum * 100 + vertical_sum;

    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let result = 0;

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};
use nom::branch::alt;
use nom::character::complete;
use nom::IResult;
use nom::multi::many1;

struct Input {
    blocks: Vec<Array2D<char>>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('#'), complete::char('.'))))(input)
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
                .map(|v| parse_string(v).unwrap().1)
                .collect_vec();
            Array2D::from_rows(&rows).unwrap()
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
        // for shift in 1..block.rows.first().map(|v| v.len()).unwrap_or(0) {
        //     let equal = block.rows_iter().all(|v| v.bytes().take(shift).rev().zip(v.bytes().skip(shift)).all(|(left, right)| left == right));
        //     if equal {
        //         vertical_sum += shift;
        //     }
        // }

        for shift in 1..block.row_len() {
            let equal = block.rows_iter().take(shift).rev().zip(block.rows_iter().skip(shift)).all(|(left, right)| left == right);
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

mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alphanumeric0;
use nom::IResult;
use nom::multi::separated_list0;

struct Input {
    sequences: Vec<Vec<String>>,
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(","), parse_string)(input)
}

// rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let sequences = reader.lines()
        .map(|v| {
            let line = v.unwrap();
            line.split(",").map(|v|v.to_string()).collect_vec()
        }).collect_vec();

    Ok(Input { sequences })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let s = input.sequences.iter().map(|seq| {
        let r: u32 = seq.iter().map(|v| {
            let mut acc = 0;
            for b in v.as_bytes() {
                acc += (*b as u32);
                acc *= 17;
                acc %= 256;
            }
            acc
        }).sum();
        r
    })
        .collect_vec();
    let result = s.iter().sum();

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

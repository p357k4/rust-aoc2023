mod main_test;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use itertools::{Itertools};
use nom::character::complete;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric0, line_ending, space1};
use nom::combinator::opt;
use nom::multi::{many0, separated_list0};
use nom::sequence::tuple;

struct Game {
    measurements: Vec<Vec<i32>>,
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

fn parse_measurement(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(" "), complete::i32)(input)
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let measurements = reader.lines()
        .map(|line| parse_measurement(line.unwrap().as_str()).unwrap().1)
        .collect_vec();

    Ok(Game { measurements })
}

fn reduce_forward(v: & mut [i32], n: usize) {
    for i in 0..n {
        v[i] = v[i+1] - v[i]
    }
}

fn reduce_forward_to_zero(v: & mut [i32]) -> i32 {
    for n in (0..v.len()).rev() {
        if v[0..n].iter().find(|&&x| x != 0).is_none() {
            return  v[n..v.len()].iter().sum()
        }
        reduce_forward(v, n)
    }

    0
}

fn part1(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut game = load(path)?;
    let result: i32 = game.measurements.iter_mut().map(|mut measurement| {
        reduce_forward_to_zero(measurement.as_mut_slice())
    }).sum();


    Ok(result)
}

fn part2(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut game = load(path)?;
    let result: i32 = game.measurements.iter_mut().map(|mut measurement| {
        measurement.reverse();
        reduce_forward_to_zero(measurement.as_mut_slice())
    }).sum();


    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

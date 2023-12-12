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
    status: Vec<char>,
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
        Row { status, groups }
    }).collect_vec();

    Ok(Input { rows })
}

fn fits(status: &Vec<char>, from: usize, group: u32) -> bool {
    if from + group as usize > status.len() {
        return false
    }

    for index in from..from + group as usize {
        if status[index] == '.' {
            return false;
        }
    }

    if ((from + group as usize) < status.len()) && status[from + group as usize] == '#' {
        return false
    }

    true
}

fn alternatives(status: &Vec<char>, from: usize, permutation: &Vec<&u32>, permutation_index: usize) -> u32 {
    if permutation_index >= permutation.len() {
        return 1;
    }

    let mut counter = 0;

    if let Some((position, &c)) = status.iter().skip(from).find_position(|&&c| c != '.') {
        let position_index = from + position;
        if c == '?' { // it may be '.'
            let d = alternatives(status, position_index + 1, permutation, permutation_index);
            counter += d;
        }

        let &p = permutation[permutation_index];
        if fits(status, position_index, p) {
            let d = alternatives(status, position_index + p as usize + 1, permutation, permutation_index + 1);
            counter += d;
        }
    }

    counter
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = game.rows.iter().map(|row| {
        let perm = row.groups.iter()
            .permutations(row.groups.len())
            .unique()
            .collect_vec()
            .iter().map(|v| v.iter().copied().collect_vec())
            .collect_vec();

        let d: u32 = perm.iter().map(|p| {
            let d = alternatives(&row.status, 0, p, 0);
            d
        }).sum();
        println!("{}", d);
        d
    }).sum();

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

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
    let sub = status.iter().skip(from).take(group as usize).collect_vec();

    if sub.len() < group as usize {
        return false
    }

    for &c in sub {
        if c == '.' {
            return false;
        }
    }

    if let Some(&c) = status.get(from + group as usize) {
        if c == '#' {
            return false
        }
    }

    true
}

fn alternatives(status: &Vec<char>, from: usize, groups: Vec<u32>) -> u32 {
    let mut counter = 0;


    if from >= status.len() && groups.is_empty() {
        return 1
    }

    for position in from..status.len() {
        let c = status[position];
        if c == '.' {
            continue;
        }

        if c == '?' { // it may be '.'
            let d = alternatives(status, position + 1, groups.clone());
            counter += d;
        }

        for group_index in 0..groups.len() {
            let r = ..group_index;


            if !fits(status, position, groups[group_index]) {
                continue;
            }

            let next_index = position + groups[group_index] as usize;

            let q = group_index + 1..;

            let next_group = groups[r].iter().chain(groups[q].iter()).copied().collect_vec();
            let d = alternatives(status, next_index + 1, next_group);
            counter += d;
        }
    }

    counter
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let g = game.rows[0].groups.iter()
        .permutations(game.rows[0].groups.len())
        .unique()
        .collect_vec();

    println!("{:?}", g);

    // let result = game.rows.iter().map(|row| alternatives(&row.status, 0, row.groups.clone())).sum();
    let result = alternatives(&game.rows[0].status, 0, game.rows[0].groups.clone());

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

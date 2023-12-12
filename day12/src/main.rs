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
    status: Vec<char>,
    groups: Vec<u64>,
}

#[derive(Eq, Hash, PartialEq)]
struct Partial {
    position_index: usize,
    group_index: usize,
}

struct Input {
    rows: Vec<Row>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('#'), complete::char('?'), complete::char('.'))))(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), complete::u64)(input)
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

fn fits(status: &Vec<char>, from: usize, group: u64) -> bool {
    let to = from + group as usize;
    if to > status.len() {
        return false;
    }

    for index in from..to {
        if status[index] == '.' {
            return false;
        }
    }

    if (to < status.len()) && (status[to] == '#') {
        return false;
    }

    true
}

fn alternatives(status: &Vec<char>, from: usize, groups: &Vec<u64>, group_index: usize, cache: &mut HashMap<Partial, u64>) -> u64 {
    if group_index >= groups.len() {
        return if status.iter().skip(from).all(|&c| c != '#') {
            1
        } else {
            0
        }
    }

    let mut counter = 0;

    if let Some((position, &c)) = status.iter().skip(from).find_position(|&&c| c != '.') {
        let position_index = from + position;

        let partial = Partial {group_index, position_index: position_index + 1};
        if let Some(&d) = cache.get(&partial) {
            counter += d;
        } else if c == '?' { // it may be '.'
            let d = alternatives(status, position_index + 1, groups, group_index, cache);
            cache.insert(partial, d);
            counter += d;
        }

        let p = groups[group_index];
        let partial = Partial {group_index: group_index + 1, position_index: position_index + p as usize + 1};
        if let Some(&d) = cache.get(&partial) {
            counter += d;
        } else if fits(status, position_index, p) {
            let d = alternatives(status, position_index + p as usize + 1, groups, group_index + 1, cache);
            cache.insert(partial, d);
            counter += d;
        }
    }

    counter
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = game.rows.iter().map(|row| {
        let mut cache = HashMap::new();
        let d = alternatives(&row.status, 0, &row.groups, 0, &mut cache);
        d
    }).sum();

    Ok(result)
}


fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = game.rows.iter().map(|row| {
        let sep = vec!['?'];

        let unfolded_status = [row.status.clone(), sep.clone(), row.status.clone(), sep.clone(), row.status.clone(), sep.clone(), row.status.clone(), sep.clone(), row.status.clone()].concat();
        let unfolded_group = row.groups.iter().cycle().take(5 * row.groups.len()).copied().collect_vec();

        let mut cache = HashMap::new();
        let d = alternatives(&unfolded_status, 0, &unfolded_group, 0, &mut cache);
        d
    }).sum();

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

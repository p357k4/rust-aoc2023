mod main_test;

use std::fs::File;
use std::io::{BufReader, Read};
use itertools::{Itertools};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alphanumeric1;
use nom::IResult;
use nom::sequence::tuple;

#[derive(Clone, Eq, PartialEq)]
struct Dig {
    direction: char,
    length: i128,
}

#[derive(Clone, Eq, PartialEq)]
struct Point {
    row: i128,
    column: i128,
}

struct Line {
    start: Point,
    end: Point,
    direction: char,
}

struct Input {
    dig_plan: Vec<Dig>,
}

fn dig(input: &str) -> IResult<&str, Dig> {
    tuple((alphanumeric1, tag(" "), complete::i128, tag(" (#"), alphanumeric1, tag(")")))(input)
        .map(|(leftover, (direction, _, length, _, color, _))| {
            (leftover, Dig {
                direction: direction.chars().next().unwrap(),
                length,
            })
        })
}

fn dig2(input: &str) -> IResult<&str, Dig> {
    tuple((alphanumeric1, tag(" "), complete::i32, tag(" (#"), alphanumeric1, tag(")")))(input)
        .map(|(leftover, (direction, _, color, _, length, _))| {

            // 0 means R, 1 means D, 2 means L, and 3 means U.
            let direction = match length.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => todo!(),
            };
            (leftover, Dig {
                direction: direction,
                length: i128::from_str_radix(&length[0..length.len()-1], 16).unwrap(),
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
            let (leftover, part) = dig(line).unwrap();
            part
        }).collect_vec();

    Ok(Input { dig_plan })
}

fn load2(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content);

    let dig_plan = content.split("\n")
        .map(|line| {
            let (leftover, part) = dig2(line).unwrap();
            part
        }).collect_vec();

    Ok(Input { dig_plan })
}

fn part1(path: &str) -> Result<i128, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut prev = Point { row: 0, column: 0 };

    let mut result = 0;
    for dig in input.dig_plan {
        let next = match dig.direction {
            'R' => Point { row: prev.row, column: prev.column + dig.length },
            'L' => Point { row: prev.row, column: prev.column - dig.length },
            'U' => Point { row: prev.row - dig.length, column: prev.column },
            'D' => Point { row: prev.row + dig.length, column: prev.column },
            _ => todo!(),
        };

        result += (prev.row + next.row) * (prev.column - next.column);
        result += dig.length;
        prev = next.clone();
    }
    Ok(result.abs() / 2 + 1)
}

fn part2(path: &str) -> Result<i128, Box<dyn std::error::Error>> {
    let input = load2(path)?;

    let mut prev = Point { row: 0, column: 0 };

    let mut result = 0;
    for dig in input.dig_plan {
        let next = match dig.direction {
            'R' => Point { row: prev.row, column: prev.column + dig.length },
            'L' => Point { row: prev.row, column: prev.column - dig.length },
            'U' => Point { row: prev.row - dig.length, column: prev.column },
            'D' => Point { row: prev.row + dig.length, column: prev.column },
            _ => todo!(),
        };

        result += (prev.row + next.row) * (prev.column - next.column);
        result += dig.length;
        prev = next.clone();
    }
    Ok(result.abs() / 2 + 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

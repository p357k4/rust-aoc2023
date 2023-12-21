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
    length: i32,
    color: String,
}

#[derive(Clone, Eq, PartialEq)]
struct Point {
    row: i32,
    column: i32,
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
    tuple((alphanumeric1, tag(" "), complete::i32, tag(" (#"), alphanumeric1, tag(")")))(input)
        .map(|(leftover, (direction, _, length, _, color, _))| {
            (leftover, Dig {
                direction: direction.chars().next().unwrap(),
                length,
                color: color.to_string(),
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

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut prev = Point { row: 0, column: 0 };
    let mut lines = vec![];

    for dig in input.dig_plan {
        let next = match dig.direction {
            'R' => Point { row: prev.row, column: prev.column + dig.length },
            'L' => Point { row: prev.row, column: prev.column - dig.length },
            'U' => Point { row: prev.row - dig.length, column: prev.column },
            'D' => Point { row: prev.row + dig.length, column: prev.column },
            _ => todo!(),
        };
        let line = Line { start: prev, end: next.clone(), direction: dig.direction };
        lines.push(line);
        prev = next.clone();
    }
    let mut result = 0;

    let verticals = lines.iter().filter(|line| line.direction == 'D' || line.direction == 'U').collect_vec();

    let rows = verticals.iter().flat_map(|line| [line.start.row, line.end.row]);

    let min_row = rows.clone().min().unwrap();
    let max_row = rows.clone().max().unwrap();

    for row in min_row..=max_row {
        let filtered = verticals.iter()
            .filter(|line| {
                (line.direction == 'D' && line.start.row <= row && row <= line.end.row)
                    || (line.direction == 'U' && line.start.row >= row && row >= line.end.row)
            })
            .sorted_by_key(|line| line.start.column)
            .collect_vec();

        let windows = filtered.windows(2).collect_vec();
        let span = windows
            .iter()
            .map(|&chunk| {
                let diff = chunk[1].start.column.abs_diff(chunk[0].start.column);
                if chunk[0].direction == chunk[1].direction {
                    diff
                } else if chunk[0].direction == 'U' && chunk[1].direction == 'D' {
                    1 + diff
                } else {
                    0
                }
            })
            .collect_vec();

        let sum: u32 = span.iter().sum();

        result += sum;
    }

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

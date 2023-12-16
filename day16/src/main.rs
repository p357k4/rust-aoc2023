mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};
use nom::branch::alt;
use nom::character::complete;
use nom::IResult;
use nom::multi::many1;


#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Step {
    point: Point,
    direction: Direction,
}

struct Input {
    grid: Array2D<char>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('|'), complete::char('-'), complete::char('/'), complete::char('\\'), complete::char('.'))))(input)
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|v| parse_string(v.unwrap().as_str()).unwrap().1).collect_vec();

    let grid = Array2D::from_rows(&lines).unwrap();

    Ok(Input { grid })
}

fn next(grid: &Array2D<char>, step: Step) -> Option<Step> {
    match step.direction {
        Direction::East if step.point.column == grid.num_columns() - 1 => None,
        Direction::West if step.point.column == 0 => None,
        Direction::North if step.point.row == 0 => None,
        Direction::South if step.point.row == grid.num_rows() - 1 => None,
        Direction::West => Some(Step { point: Point { row: step.point.row, column: step.point.column - 1 }, direction: step.direction }),
        Direction::East => Some(Step { point: Point { row: step.point.row, column: step.point.column + 1 }, direction: step.direction }),
        Direction::North => Some(Step { point: Point { row: step.point.row - 1, column: step.point.column }, direction: step.direction }),
        Direction::South => Some(Step { point: Point { row: step.point.row + 1, column: step.point.column }, direction: step.direction }),
    }
}

fn reflect(grid: &Array2D<char>, step: Step, c: char) -> Option<Step> {
    match step.direction {
        Direction::East if c == '\\' => next(grid, Step { direction: Direction::South, ..step }),
        Direction::East if c == '/' => next(grid, Step { direction: Direction::North, ..step }),
        Direction::West if c == '\\' => next(grid, Step { direction: Direction::North, ..step }),
        Direction::West if c == '/' => next(grid, Step { direction: Direction::South, ..step }),
        Direction::North if c == '\\' => next(grid, Step { direction: Direction::West, ..step }),
        Direction::North if c == '/' => next(grid, Step { direction: Direction::East, ..step }),
        Direction::South if c == '\\' => next(grid, Step { direction: Direction::East, ..step }),
        Direction::South if c == '/' => next(grid, Step { direction: Direction::West, ..step }),
        _ => None,
    }
}

fn split(grid: &Array2D<char>, step: Step, c: char) -> Vec<Option<Step>> {
    match step.direction {
        Direction::East if c == '|' =>
            vec![
                next(grid, Step { direction: Direction::South, point: step.point.clone() }),
                next(grid, Step { direction: Direction::North, point: step.point }),
            ],
        Direction::East if c == '-' => vec![next(grid, step)],
        Direction::West if c == '|' =>
            vec![
                next(grid, Step { direction: Direction::South, point: step.point.clone() }),
                next(grid, Step { direction: Direction::North, point: step.point }),
            ],
        Direction::West if c == '-' => vec![next(grid, step)],

        Direction::North if c == '|' => vec![next(grid, step)],
        Direction::North if c == '-' => vec![
            next(grid, Step { direction: Direction::East, point: step.point.clone() }),
            next(grid, Step { direction: Direction::West, point: step.point }),
        ],
        Direction::South if c == '|' => vec![next(grid, step)],
        Direction::South if c == '-' => vec![
            next(grid, Step { direction: Direction::East, point: step.point.clone() }),
            next(grid, Step { direction: Direction::West, point: step.point }),
        ],
        _ => vec![],
    }
}

fn light(grid: &Array2D<char>, step: Step, path: &mut Vec<Step>) {
    let c = grid.get(step.point.row, step.point.column).unwrap();

    if path.contains(&step) {
        return;
    }
    path.push(step.clone());

    let next_step_options = if *c == '.' {
        vec![next(grid, step)]
    } else if *c == '\\' || *c == '/' {
        vec![reflect(grid, step, *c)]
    } else if *c == '|' || *c == '-' {
        split(grid, step, *c)
    } else {
        vec![]
    };

    for next_step_option in next_step_options {
        if let Some(next_step) = next_step_option {
            light(grid, next_step, path)
        }
    }
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut path = vec![];

    light(&input.grid, Step {
        point: Point {
            row: 0,
            column: 0,
        },
        direction: Direction::East,
    }, &mut path);

    let result = path.iter().unique_by(|p| p.point.clone()).count();

    Ok(result)
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut result = 0;

    for column in 0..input.grid.num_columns() {
        let mut path = vec![];
        light(&input.grid, Step {
            point: Point {
                row: 0,
                column: column,
            },
            direction: Direction::South,
        }, &mut path);
        let r = path.iter().unique_by(|p| p.point.clone()).count();

        if r > result {
            result = r;
        }
    }

    for column in 0..input.grid.num_columns() {
        let mut path = vec![];
        light(&input.grid, Step {
            point: Point {
                row: input.grid.num_columns() - 1,
                column: column,
            },
            direction: Direction::North,
        }, &mut path);
        let r = path.iter().unique_by(|p| p.point.clone()).count();

        if r > result {
            result = r;
        }
    }

    for row in 0..input.grid.num_rows() {
        let mut path = vec![];
        light(&input.grid, Step {
            point: Point {
                row: row,
                column: 0,
            },
            direction: Direction::East,
        }, &mut path);
        let r = path.iter().unique_by(|p| p.point.clone()).count();

        if r > result {
            result = r;
        }
    }

    for row in 0..input.grid.num_rows() {
        let mut path = vec![];
        light(&input.grid, Step {
            point: Point {
                row: row,
                column: input.grid.num_columns() - 1,
            },
            direction: Direction::West,
        }, &mut path);
        let r = path.iter().unique_by(|p| p.point.clone()).count();

        if r > result {
            result = r;
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

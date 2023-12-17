mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::IResult;
use nom::multi::many1;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Path {
    trace: Vec<Point>,
}

struct Input {
    grid: Array2D<u32>,
}

fn parse_string(input: &str) -> IResult<&str, Vec<char>> {
    many1(alt((complete::char('|'), complete::char('-'), complete::char('/'), complete::char('\\'), complete::char('.'))))(input)
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .map(|v| v.unwrap())
        .collect_vec();
    let digits = lines.iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let grid = Array2D::from_rows(&digits).unwrap();

    Ok(Input { grid })
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn next(grid: &Array2D<u32>, p0: &Point, direction: Direction) -> Option<Point> {
    match direction {
        Direction::North if p0.row > 0 => Some(Point { row: p0.row - 1, column: p0.column }),
        Direction::South if p0.row < grid.num_rows() - 1 => Some(Point { row: p0.row + 1, column: p0.column }),
        Direction::East if p0.column < grid.num_columns() - 1 => Some(Point { row: p0.row, column: p0.column + 1 }),
        Direction::West if p0.column > 0 => Some(Point { row: p0.row, column: p0.column - 1 }),
        _ => None
    }
}

fn cost(grid: &Array2D<u32>, path: &Path) -> u32 {
    path.trace.iter().map(|p| grid.get(p.row, p.column).unwrap()).sum()
}

fn roll(grid: &Array2D<u32>, paths: &Vec<Path>) -> Vec<Path> {
    let np = paths.iter().flat_map(|path| {
        let p0 = path.trace.first().unwrap();

        if p0.row == grid.num_rows() - 1 && p0.column == grid.num_columns() - 1 {
            vec![]
        } else {
            let next_options_vec = vec![
                next(grid, p0, Direction::West),
                next(grid, p0, Direction::East),
                next(grid, p0, Direction::North),
                next(grid, p0, Direction::South),
            ];

            next_options_vec
                .iter()
                .flatten()
                .filter(|&next| !path.trace.contains(next))
                .filter(|&next| path.trace.iter().take(3).filter(|p| p.column == next.column).count() < 3)
                .filter(|&next| path.trace.iter().take(3).filter(|p| p.row == next.row).count() < 3)
                .map(|next| {
                    let mut trace = path.trace.clone();
                    trace.insert(0, next.clone());
                    Path { trace }
                })
                .collect_vec()
        }
    })
        .collect_vec();

    roll(grid, &np)
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let p1 = Point { row: 0, column: 0 };
    let initial = vec![Path { trace: vec![p1] }];
    let all= roll(&input.grid, &initial);

    let result = 0;
    Ok(result)
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

mod main_test;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;
use itertools::{Itertools};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
enum PuzzleError {
    Input,
}

impl Error for PuzzleError {}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


struct Input {
    grid: Array2D<char>,
    start: Point,
}

fn load(path: &str) -> Result<Input, PuzzleError> {
    let file = File::open(path).map_err(|v| PuzzleError::Input)?;
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .flatten()
        .map(|v|v.chars().collect_vec())
        .collect_vec();

    let grid = Array2D::from_rows(&lines).unwrap();

    for row in 0..grid.num_columns() {
        for column in 0.. grid.num_columns() {
            if *grid.get(row, column).unwrap() == 'S' {
                let start = Point{row, column};
                return Ok(Input { grid, start })
            }
        }
    }

    Err(PuzzleError::Input)
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn next(grid: &Array2D<char>, p1: &Point, direction: Direction) -> Option<Point> {
    match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    }
}

fn walk(grid: &Array2D<char>, steps: &mut Array2D<u32>, start: Point, depth: usize) -> usize {

    let mut firsts = vec![start];

    for i in 0..depth {
        let mut nexts = vec![];
        for p0 in &firsts {
            let p0_steps = *steps.get(p0.row, p0.column).unwrap();

            let next_options_vec = [
                next(grid, p0, Direction::South),
                next(grid, p0, Direction::North),
                next(grid, p0, Direction::East),
                next(grid, p0, Direction::West),
            ];

            for next in next_options_vec.iter().flatten() {
                if nexts.contains(next) {
                    continue
                }

                let block = *grid.get(next.row, next.column).unwrap();
                if block == '#' {
                    continue
                }

                let next_steps = *steps.get(next.row, next.column).unwrap();
                if next_steps > p0_steps + 1 {
                    continue
                }

                *steps.get_mut(next.row, next.column).unwrap() = p0_steps + 1;
                nexts.push(next.clone())
            }
        }

        firsts = nexts;
    }

    firsts.len()
}

fn part1(path: &str, depth: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_column_major(|| 0, input.grid.num_rows(), input.grid.num_columns());

    let result = walk(&input.grid, &mut steps, input.start, depth);
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

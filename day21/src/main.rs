mod main_test;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use array2d::Array2D;
use itertools::{Itertools};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    row: i32,
    column: i32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct PointSteps {
    point: Point,
    steps: usize,
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
        .map(|v| v.chars().collect_vec())
        .collect_vec();

    let grid = Array2D::from_rows(&lines).unwrap();

    for row in 0..grid.num_columns() {
        for column in 0..grid.num_columns() {
            if *grid.get(row, column).unwrap() == 'S' {
                let start = Point { row: row as i32, column: column as i32 };
                return Ok(Input { grid, start });
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
        Direction::South if p1.row < (grid.num_rows() - 1) as i32 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < (grid.num_columns() - 1) as i32 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    }
}

fn walk(grid: &Array2D<char>, steps: &mut Array2D<u32>, start: Point, depth: usize) -> usize {
    let mut firsts = vec![start];

    for i in 0..depth {
        let mut nexts = vec![];
        for p0 in &firsts {
            let p0_steps = *steps.get(p0.row as usize, p0.column as usize).unwrap();

            let next_options_vec = [
                next(grid, p0, Direction::South),
                next(grid, p0, Direction::North),
                next(grid, p0, Direction::East),
                next(grid, p0, Direction::West),
            ];

            for next in next_options_vec.iter().flatten() {
                if nexts.contains(next) {
                    continue;
                }

                let block = *grid.get(next.row as usize, next.column as usize).unwrap();
                if block == '#' {
                    continue;
                }

                // let next_steps = *steps.get(next.row, next.column).unwrap();
                // if next_steps > p0_steps + 1 {
                //     continue
                // }
                //
                // *steps.get_mut(next.row, next.column).unwrap() = p0_steps + 1;
                nexts.push(next.clone())
            }
        }

        firsts = nexts;
    }

    firsts.len()
}

fn infinite_next(grid: &Array2D<char>, p1: &Point, direction: Direction) -> Point {
    match direction {
        Direction::North if p1.row > 0 => Point { row: p1.row - 1, column: p1.column },
        Direction::South if p1.row < (grid.num_rows() - 1) as i32 => Point { row: p1.row + 1, column: p1.column },
        Direction::East if p1.column < (grid.num_columns() - 1) as i32 => Point { row: p1.row, column: p1.column + 1 },
        Direction::West if p1.column > 0 => Point { row: p1.row, column: p1.column - 1 },
        Direction::North => Point { row: p1.row - 1, column: p1.column },
        Direction::South => Point { row: p1.row + 1, column: p1.column },
        Direction::East=> Point { row: p1.row, column: p1.column + 1 },
        Direction::West => Point { row: p1.row, column: p1.column - 1 },
    }
}

fn infinite_walk(grid: &Array2D<char>, cache: &mut HashMap<PointSteps, usize>, p0: &Point, depth: usize) -> usize {
    if depth == 0 {
        return 1
    }

    let mut sum = 0;

    let key = PointSteps{point: p0.clone(), steps: depth};
    let p0_steps_option = cache.get(&key);
    if let Some(&p0_steps) = p0_steps_option {
        return p0_steps
    }

    let mut firsts = vec![p0.clone()];

    for i in 0..depth {
        let mut nexts = vec![];
        for p0 in &firsts {
            let next_options_vec = [
                infinite_next(grid, p0, Direction::South),
                infinite_next(grid, p0, Direction::North),
                infinite_next(grid, p0, Direction::East),
                infinite_next(grid, p0, Direction::West),
            ];

            for next in next_options_vec.iter() {
                if nexts.contains(next) {
                    continue;
                }

                let row = (next.row.rem_euclid(grid.num_columns() as i32)) as usize;
                let column = (next.column.rem_euclid(grid.num_columns() as i32))as usize;
                let block = *grid.get(row, column).unwrap();
                if block == '#' {
                    continue;
                }

                nexts.push(next.clone())
            }
        }

        firsts = nexts;
    }

    sum += firsts.len();

    cache.insert(key, sum);
    sum
}

fn part1(path: &str, depth: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_column_major(|| 0, input.grid.num_rows(), input.grid.num_columns());

    let result = walk(&input.grid, &mut steps, input.start, depth);
    Ok(result)
}

fn part2(path: &str, depth: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut cache = HashMap::new();
    let result = infinite_walk(&input.grid, &mut cache, &input.start, depth);
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

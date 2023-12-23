mod main_test;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use array2d::Array2D;
use itertools::{Itertools};

struct Input {
    grid: Array2D<char>,
    start: Point,
}

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


fn load(path: &str) -> Result<Input, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .flatten()
        .map(|v| v.chars().collect_vec())
        .collect_vec();

    let grid = Array2D::from_rows(&lines).unwrap();

    for row in 0..grid.num_columns() {
        for column in 0..grid.num_columns() {
            if *grid.get(row, column).unwrap() == 'S' {
                let start = Point { row, column };
                return Ok(Input { grid, start });
            }
        }
    }

    todo!("we should never be here")
}
fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_row_major(|| 0, input.grid.num_rows(), input.grid.num_columns());

    walk(&input.grid, &mut steps, &input.start, 1);

    let result = steps.elements_row_major_iter().max().unwrap();
    Ok(*result)
}

fn walk(grid: &Array2D<char>, steps: &mut Array2D<usize>, p: &Point, depth: usize) {
    let Some(c) = grid.get(p.row, p.column) else { return  };

    if *c == '#' {
        return
    }

    let ps = *steps.get(p.row, p.column).unwrap();
    if ps > depth {
        return
    }
    *steps.get_mut(p.row, p.column).unwrap() = depth;

    let ds = directions(grid, &p);
    let nos = ds.iter().map(|d| next(grid, &p, d)).collect_vec();

    for n in nos.iter().flatten() {
        walk(grid, steps, n, depth + 1)
    }
}

fn directions(grid: &Array2D<char>, p1: &Point) -> Vec<Direction> {
    let Some(c) = grid.get(p1.row, p1.column) else { return vec![] };

    match c {
        'S' | '.' => vec![Direction::North, Direction::South, Direction::West, Direction::East],
        '>' => vec![Direction::East],
        '<' => vec![Direction::West],
        'v' => vec![Direction::South],
        _ => vec![],
    }
}


fn next(grid: &Array2D<char>, p1: &Point, direction: &Direction) -> Option<Point> {
    match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    }
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

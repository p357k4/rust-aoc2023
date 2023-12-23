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
    end: Point,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

    let mut start = Point{row:0,column:0};
    let mut end= Point{row:0,column:0};

    for column in 0..grid.num_columns() {
        if *grid.get(0, column).unwrap() == '.' {
            start = Point { row: 0, column };
            break
        }
    }

    for column in 0..grid.num_columns() {
        if *grid.get(grid.num_rows() - 1, column).unwrap() == '.' {
            end = Point { row: grid.num_rows() - 1, column };
            break
        }
    }

    return Ok(Input { grid, start, end });

    todo!("we should never be here")
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_row_major(|| 0, input.grid.num_rows(), input.grid.num_columns());

    let ps = vec![input.start];
    walk(&input, &mut steps, &ps, 1, &directions);

    let result = steps.get(input.end.row, input.end.column).unwrap() - steps.get(input.start.row, input.start.column).unwrap();
    Ok(result)
}

fn walk(input: &Input, steps: &mut Array2D<usize>, path: &Vec<Point>, depth: usize, directions: &impl Fn(&Array2D<char>, &Point) -> Vec<Direction>) {
    let Some(p) = path.first() else { return };

    let Some(c) = input.grid.get(p.row, p.column) else { return; };

    if *c == '#' {
        return;
    }

    let ps = *steps.get(p.row, p.column).unwrap();
    if ps > depth {
        return;
    }
    *steps.get_mut(p.row, p.column).unwrap() = depth;

    if *p == input.end {
        return
    }

    let ds = directions(&input.grid, &p);
    let nos = ds.iter().map(|d| next(&input.grid, &p, d)).collect_vec();

    for n in nos.iter().flatten() {
        if path.contains(&n) {
            continue
        }

        let mut nps = path.clone();
        nps.insert(0, n.clone());
        walk(input, steps, &nps, depth + 1, directions)
    }
}

fn directions(grid: &Array2D<char>, p1: &Point) -> Vec<Direction> {
    let Some(c) = grid.get(p1.row, p1.column) else { return vec![]; };

    match c {
        '.' => vec![Direction::North, Direction::South, Direction::West, Direction::East],
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

    let mut steps = Array2D::filled_by_row_major(|| 0, input.grid.num_rows(), input.grid.num_columns());

    let ps = vec![input.start];
    walk(&input, &mut steps, &ps, 1, &directions2);

    let result = steps.get(input.end.row, input.end.column).unwrap() - steps.get(input.start.row, input.start.column).unwrap();
    Ok(result)
}

fn directions2(grid: &Array2D<char>, p1: &Point) -> Vec<Direction> {
    vec![Direction::North, Direction::South, Direction::West, Direction::East]
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

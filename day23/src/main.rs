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

    let mut start = Point { row: 0, column: 0 };
    let mut end = Point { row: 0, column: 0 };

    for column in 0..grid.num_columns() {
        if *grid.get(0, column).unwrap() == '.' {
            start = Point { row: 0, column };
            break;
        }
    }

    for column in 0..grid.num_columns() {
        if *grid.get(grid.num_rows() - 1, column).unwrap() == '.' {
            end = Point { row: grid.num_rows() - 1, column };
            break;
        }
    }

    return Ok(Input { grid, start, end });

    todo!("we should never be here")
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_row_major(|| 1, input.grid.num_rows(), input.grid.num_columns());

    let ps = vec![input.start, Point { row: input.start.row + 1, column: input.start.column }];
    let result = walk(&input, &mut steps, &ps, &directions);

    // let result = steps.get(input.end.row, input.end.column).unwrap() - steps.get(input.start.row, input.start.column).unwrap();
    Ok(result)
}

fn walk(input: &Input, steps: &mut Array2D<usize>, path: &Vec<Point>, directions: &impl Fn(&Array2D<char>, &Point) -> Vec<Direction>) -> usize {
    let mut new_path = path.clone();
    loop {
        let Some(p) = new_path.last() else { todo!() };

        if *p == input.end {
            return new_path.len() - 1;
        }

        // if *steps.get_mut(p.row, p.column).unwrap() == 0 {
        //     return 0;
        // }

        let ds = directions(&input.grid, p);

        let mut ns = ds.iter()
            .map(|d| next(p, d))
            .filter(|n| matches!(input.grid.get(n.row, n.column), Some(c) if *c != '#'))
            .filter(|n| *n != new_path[new_path.len() - 2])
            .collect_vec();

        if ns.is_empty() {
            *steps.get_mut(p.row, p.column).unwrap() = 0;
            break;
        }

        if ns.len() == 1 {
            let n = ns.last().unwrap();
            new_path.push(*n);
            continue;
        }

        let mo = ns.iter()
            .filter(|n| !new_path.contains(n))
            .map(|n| {
                let mut nps = new_path.clone();
                nps.push(*n);
                walk(input, steps, &nps, directions)
            })
            .max();

        let m = mo.unwrap_or_default();
        if m == 0 {
            *steps.get_mut(p.row, p.column).unwrap() = 0;
        }
        return m;
    }

    0
}

fn directions(grid: &Array2D<char>, p1: &Point) -> Vec<Direction> {
    match grid.get(p1.row, p1.column) {
        Some('.') => vec![Direction::North, Direction::South, Direction::West, Direction::East],
        Some('^') => vec![Direction::North],
        Some('>') => vec![Direction::East],
        Some('<') => vec![Direction::West],
        Some('v') => vec![Direction::South],
        _ => vec![],
    }
}


fn next(p1: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::North => Point { row: p1.row - 1, column: p1.column },
        Direction::South => Point { row: p1.row + 1, column: p1.column },
        Direction::East => Point { row: p1.row, column: p1.column + 1 },
        Direction::West => Point { row: p1.row, column: p1.column - 1 },
        _ => todo!("we should never be here")
    }
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut steps = Array2D::filled_by_row_major(|| 1, input.grid.num_rows(), input.grid.num_columns());

    let ps = vec![input.start, Point { row: input.start.row + 1, column: input.start.column }];
    let result = walk(&input, &mut steps, &ps, &directions2);

    // let result = steps.get(input.end.row, input.end.column).unwrap() - steps.get(input.start.row, input.start.column).unwrap();
    Ok(result)
}

fn directions2(grid: &Array2D<char>, p1: &Point) -> Vec<Direction> {
    vec![Direction::North, Direction::South, Direction::West, Direction::East]
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

mod main_test;

use std::env::join_paths;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::IResult;
use nom::multi::many1;

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    row: usize,
    column: usize,
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

fn next(grid: &Array2D<u32>, path: &[Point], depth: usize, direction: Direction) -> Option<Point> {
    let p1= &path[depth];

    let next_option = match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    };

    next_option
        .filter(|next| depth < 2 || !(next.column == p1.column && next.column == path[depth - 1].column && next.column == path[depth - 2].column))
        .filter(|next| depth < 2 || !(next.row == p1.row && next.row == path[depth - 1].row && next.row == path[depth - 2].row))
}

fn dist(grid: &Array2D<u32>, p1: &Point) -> usize {
    p1.row.abs_diff(grid.num_columns()) + p1.column.abs_diff(grid.num_rows())
}

fn roll(grid: &Array2D<u32>, cost: &mut Array2D<u32>, path_cost: u32, path: &mut Vec<Point>, depth: usize) {
    if depth > 450 {
        return;
    }

    let p1= path[depth];

    let new_cost = path_cost + *grid.get(p1.row, p1.column).unwrap();
    let end_cost = *cost.get(cost.num_rows() - 1, cost.num_columns() - 1).unwrap();
    if new_cost > end_cost {
        return;
    }

    let next_options_vec = [
        next(grid, path, depth, Direction::East),
        next(grid, path, depth, Direction::West),
        next(grid, path, depth, Direction::South),
        next(grid, path, depth, Direction::North),
    ];

    for next in next_options_vec.iter().flatten() {
        if path[0..depth].contains(next) {
            continue; // don't want to go back
        }

        if depth > 0 {
            let h = path[depth - 1];
            if (p1.column == h.column && p1.column != next.column) || (p1.row == h.row && p1.row != next.row) {
                if new_cost <= *cost.get(p1.row, p1.column).unwrap() {
                    *cost.get_mut(p1.row, p1.column).unwrap() = new_cost
                } else {
                    continue
                }
            }
        } else {
            if new_cost <= *cost.get(p1.row, p1.column).unwrap() {
                *cost.get_mut(p1.row, p1.column).unwrap() = new_cost
            } else {
                continue
            }
        }

        if p1.row == grid.num_rows() - 1 && p1.column == grid.num_columns() - 1 {
            continue
        }

        path[depth + 1] = *next;

        roll(grid, cost, new_cost, path, depth + 1);
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut cost = Array2D::filled_by_column_major(|| 1_000_000_000, input.grid.num_rows(), input.grid.num_columns());

    let p1 = Point { row: 0, column: 0 };
    let mut path = vec![p1; 500];
    roll(&input.grid, &mut cost, 0, &mut path, 0);

    let result = *cost.get(cost.num_rows() - 1, cost.num_columns() - 1).unwrap();

    for i in 0..cost.num_rows() {
        for j in 0..cost.num_columns() {
            let e = cost.get(i, j).unwrap();
            print!("\t{e}");
        }
        println!();
    }

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

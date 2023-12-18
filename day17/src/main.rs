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

#[derive(Clone, Eq, PartialEq, Hash)]
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

fn next(grid: &Array2D<u32>, p1: &Point, path: &Vec<Point>, direction: Direction) -> Option<Point> {
    let p0_option = match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    };

    p0_option
        .filter(|p0| !path.contains(p0))
        .filter(|p0| path.iter().take(3).filter(|p| p.column == p0.column).count() < 3)
        .filter(|p0| path.iter().take(3).filter(|p| p.row == p0.row).count() < 3)
}

fn roll(grid: &Array2D<u32>, cost: &mut Array2D<u32>, path_cost: u32, p1: &Point, path: Vec<Point>) {
    let new_cost = path_cost + *grid.get(p1.row, p1.column).unwrap();

    let next_options_vec = [
        next(grid, p1, &path, Direction::South),
        next(grid, p1, &path, Direction::North),
        next(grid, p1, &path, Direction::East),
        next(grid, p1, &path, Direction::West),
    ];

    for next in next_options_vec.iter().flatten() {
        if path.contains(next) {
            continue
        }

        let mut new_path = path.clone();
        new_path.insert(0, p1.clone());
        let h = new_path.iter().take(2).collect_vec();
        if h.len() == 2 {
            if (h[0].column == h[1].column && h[0].column != next.column) || (h[0].row == h[1].row && h[0].row != next.row) {
                if new_cost <= *cost.get(p1.row, p1.column).unwrap() {
                    *cost.get_mut(p1.row, p1.column).unwrap() = new_cost
                } else {
                    continue
                }
            }
        }

        if p1.row == grid.num_rows() - 1 && p1.column == grid.num_columns() - 1{
            println!("{new_cost}");
            continue
        }

        roll(grid, cost, new_cost, next, new_path);
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut cost = Array2D::filled_by_column_major(|| 1_000_000_000, input.grid.num_rows(), input.grid.num_columns());

    let p1 = Point { row: 0, column: 0 };
    roll(&input.grid, &mut cost, 0, &p1, vec![]);

    let result = *cost.get(cost.num_rows() - 1, cost.num_columns() - 1).unwrap() - *input.grid.get(0, 0).unwrap();

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

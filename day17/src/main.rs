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

fn next(grid: &Array2D<u32>, p1: &Point, direction: Direction) -> Option<Point> {
    match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    }
}

fn roll(grid: &Array2D<u32>, current_cost: &mut Array2D<u32>, p1: &Point, tail: &Vec<Point>, path_cost: u32) {
    let new_cost = path_cost + *grid.get(p1.row, p1.column).unwrap();

    if new_cost < *current_cost.get(p1.row, p1.column).unwrap() {
        *current_cost.get_mut(p1.row, p1.column).unwrap() = new_cost;

        if p1.row == grid.num_rows() - 1 && p1.column == grid.num_columns() - 1 {
            println!("{:?}", tail);
            return
        }

        let next_options_vec = vec![
            next(grid, p1, Direction::West),
            next(grid, p1, Direction::East),
            next(grid, p1, Direction::North),
            next(grid, p1, Direction::South),
        ];

        for next in next_options_vec.iter().flatten() {
            let mut new_tail= tail.clone();
            new_tail.insert(0, p1.clone());

            let column_counter = new_tail.iter().take(2).filter(|p| p.column == next.column).count();
            if column_counter == 2 {
                continue
            }

            let row_counter = new_tail.iter().take(2).filter(|p| p.row == next.row).count();
            if row_counter == 2 {
                continue
            }

            roll(grid, current_cost, next, &new_tail, new_cost);
        }
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut current_cost = Array2D::filled_by_column_major(|| 1_000, input.grid.num_rows(), input.grid.num_columns());

    let p1 = Point { row: 0, column: 0 };
    let tail = vec![];
    roll(&input.grid, &mut current_cost, &p1, &tail, 0);

    let result = *current_cost.get(current_cost.num_rows() - 1, current_cost.num_columns() - 1).unwrap();

    for i in 0..current_cost.num_rows() {
        for j in 0..current_cost.num_columns() {
            let e = current_cost.get(i, j).unwrap();
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

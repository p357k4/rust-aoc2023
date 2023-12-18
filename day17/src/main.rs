mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    column: usize,
}

struct Input {
    grid: Array2D<u32>,
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

fn next(grid: &Array2D<u32>, path: &Vec<Point>, direction: Direction) -> Option<Point> {
    let depth = path.len() - 1;
    let p1 = &path[depth];
    let p0_option = match direction {
        Direction::North if p1.row > 0 => Some(Point { row: p1.row - 1, column: p1.column }),
        Direction::South if p1.row < grid.num_rows() - 1 => Some(Point { row: p1.row + 1, column: p1.column }),
        Direction::East if p1.column < grid.num_columns() - 1 => Some(Point { row: p1.row, column: p1.column + 1 }),
        Direction::West if p1.column > 0 => Some(Point { row: p1.row, column: p1.column - 1 }),
        _ => None
    };

    p0_option
        .filter(|p0| !path.contains(p0))
        .filter(|p0| depth < 3 || !(p0.column == path[depth].column && p0.column == path[depth - 1].column && p0.column == path[depth - 2].column && p0.column == path[depth - 3].column))
        .filter(|p0| depth < 3 || !(p0.row == path[depth].row && p0.row == path[depth - 1].row && p0.row == path[depth - 2].row && p0.row == path[depth - 3].row))
}

fn roll(grid: &Array2D<u32>, cost: &mut Array2D<u32>, path_cost: u32, p1: &Point, path: &Vec<Point>) {
    let new_cost = path_cost + *grid.get(p1.row, p1.column).unwrap();

    if path.len() > 450 {
        return;
    }

    if new_cost > *cost.get(grid.num_rows() - 1, grid.num_columns() - 1).unwrap() {
        return;
    }

    let mut new_path = path.clone();
    new_path.push(p1.clone());

    let next_options_vec = [
        next(grid, &new_path, Direction::South),
        next(grid, &new_path, Direction::North),
        next(grid, &new_path, Direction::East),
        next(grid, &new_path, Direction::West),
    ];

    for next in next_options_vec.iter().flatten() {
        if path.contains(next) {
            continue;
        }

        // if path.len() > 0 {
        //     if (p1.column == path[0].column && p1.column != next.column) || (p1.row == path[0].row && p1.row != next.row) {
        //         if new_cost <= *cost.get(p1.row, p1.column).unwrap() {
        //             *cost.get_mut(p1.row, p1.column).unwrap() = new_cost
        //         } else {
        //             continue
        //         }
        //     }
        // }

        if new_cost <= *cost.get(p1.row, p1.column).unwrap() {
            *cost.get_mut(p1.row, p1.column).unwrap() = new_cost
        } else {
            continue;
        }

        if p1.row == grid.num_rows() - 1 && p1.column == grid.num_columns() - 1 {
            println!("{new_cost}");
            continue;
        }

        roll(grid, cost, new_cost, next, &new_path);
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut cost = Array2D::filled_by_column_major(|| 1_000_000_000, input.grid.num_rows(), input.grid.num_columns());

    let p1 = Point { row: 0, column: 0 };
    roll(&input.grid, &mut cost, 0, &p1, &vec![]);

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

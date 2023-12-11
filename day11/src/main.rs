mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use array2d::Array2D;
use itertools::{Itertools};

#[derive(Clone, Copy, PartialEq)]
struct Point {
    row: usize,
    column: usize,
}

struct Game {
    board: Array2D<char>,
    expanded_rows: Vec<usize>,
    expanded_columns: Vec<usize>,
    galaxies: Vec<Point>,
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap().chars().collect_vec()).collect_vec();

    let board = Array2D::from_rows(&lines).unwrap();

    let mut expanded_rows = vec![];
    let mut expanded_columns = vec![];
    let mut galaxies = vec![];

    for row_index in 0..board.num_rows() {
        let x_option = board.row_iter(row_index).unwrap().find(|&&x| x != '.');
        if x_option.is_some() {
            continue;
        }
        expanded_rows.push(row_index)
    }

    for column_index in 0..board.num_columns() {
        let x_option = board.column_iter(column_index).unwrap().find(|&&x| x != '.');
        if x_option.is_some() {
            continue;
        }
        expanded_columns.push(column_index)
    }

    for row in 0..board.num_rows() {
        for column in 0..board.num_columns() {
            let item = board.get(row, column).unwrap();
            if *item != '#' {
                continue;
            }
            galaxies.push(Point { row, column })
        }
    }

    Ok(Game { board, expanded_rows, expanded_columns, galaxies })
}

fn distance(game: &Game, start: &Point, end: &Point, expansion: u64) -> u64 {
    let mut cost = 0;
    for row in start.row..end.row {
        cost += if game.expanded_rows.contains(&row) {
            expansion
        } else {
            1
        }
    }
    for row in end.row..start.row {
        cost += if game.expanded_rows.contains(&row) {
            expansion
        } else {
            1
        }
    }
    for column in start.column..end.column {
        cost += if game.expanded_columns.contains(&column) {
            expansion
        } else {
            1
        }
    }
    for column in end.column..start.column {
        cost += if game.expanded_columns.contains(&column) {
            expansion
        } else {
            1
        }
    }

    cost
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let combinations = game.galaxies.iter().combinations(2);
    let costs = combinations.map(|c| {
        let &left = c.first().unwrap();
        let &right = c.last().unwrap();
        let cost = distance(&game, left, right, 2);
        cost
    }).collect_vec();

    let result = costs.iter().sum();
    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let combinations = game.galaxies.iter().combinations(2);
    let costs = combinations.map(|c| {
        let &left = c.first().unwrap();
        let &right = c.last().unwrap();
        let cost = distance(&game, left, right, 1_000_000);
        cost
    }).collect_vec();

    let result = costs.iter().sum();
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

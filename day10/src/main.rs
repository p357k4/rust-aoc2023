mod main_test;

use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use array2d::Array2D;
use itertools::{Itertools};
use crate::Direction::{East, North, South, West};

struct Game {
    board: Array2D<char>,
}

#[derive(Clone, PartialEq)]
struct Pipe {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap().chars().collect_vec()).collect_vec();

    let board = Array2D::from_rows(&lines).unwrap();

    Ok(Game { board })
}

fn path_length(game: &Game, mut row: usize, mut column: usize, mut from_direction: Direction) -> Vec<Pipe> {
    let directions = HashMap::from([
        ('|', vec![North, South]),
        ('-', vec![East, West]),
        ('L', vec![North, East]),
        ('J', vec![North, West]),
        ('7', vec![South, West]),
        ('F', vec![South, East]),
        ('S', vec![North, South, East, West]),
    ]);

    let mut points = vec![];

    loop {
        let item_option = game.board.get(row, column);

        if item_option.is_none() {
            return vec![];
        }

        let item = item_option.unwrap();

        if *item == '.' {
            return vec![];
        }

        if *item == 'S' {
            return points;
        }
        points.push(Pipe { row, column });

        let available_directions = directions.get(item).unwrap();

        let to_direction = available_directions.iter().find(|&direction| *direction != from_direction).unwrap();

        if *to_direction == North {
            row -= 1;
            from_direction = South;
            continue;
        }

        if *to_direction == South {
            row += 1;
            from_direction = North;
            continue;
        }

        if *to_direction == East {
            column += 1;
            from_direction = West;
            continue;
        }

        if *to_direction == West {
            column -= 1;
            from_direction = East;
            continue;
        }
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let mut row_s = 0;
    let mut column_s = 0;
    for row in 0..game.board.num_rows() {
        for column in 0..game.board.num_columns() {
            let item = game.board.get(row, column).unwrap();
            if *item == 'S' {
                row_s = row;
                column_s = column;
            }
        }
    }

    let longest = longest_path(&game, row_s, column_s);
    let max = longest.len();
    let result = (max / 2) + (max & 1);
    Ok(result as u32)
}

fn longest_path(game: &Game, mut row_s: usize, mut column_s: usize) -> Vec<Pipe> {
    let mut max = 0;
    let mut result = vec![];
    if row_s > 0 {
        let path = path_length(&game, row_s - 1, column_s, South);
        if path.len() > max {
            max = path.len();
            result = path;
        }
    }
    if row_s < game.board.num_rows() - 1 {
        let path = path_length(&game, row_s + 1, column_s, North);
        if path.len() > max {
            max = path.len();
            result = path;
        }
    }
    if column_s > 0 {
        let path = path_length(&game, row_s, column_s - 1, West);
        if path.len() > max {
            max = path.len();
            result = path;
        }
    }
    if column_s < game.board.num_columns() - 1 {
        let path = path_length(&game, row_s, column_s + 1, East);
        if path.len() > max {
            max = path.len();
            result = path;
        }
    }

    result.push(Pipe { row: row_s, column: column_s });

    result
}

fn color(mut game: &Game, row: usize, column: usize) {
    let item = game.board.get(row, column).unwrap();
}

fn part2(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut game = load(path)?;

    let mut row_s = 0;
    let mut column_s = 0;
    for row in 0..game.board.num_rows() {
        for column in 0..game.board.num_columns() {
            let item = game.board.get(row, column).unwrap();
            if *item == 'S' {
                row_s = row;
                column_s = column;
            }
        }
    }

    let longest = longest_path(&game, row_s, column_s);

    for row in 0..game.board.num_rows() {
        for column in 0..game.board.num_columns() {
            let mut maybe = vec![];
            let l = leakage(&mut game.board, &longest, &mut maybe, row, column);

            if l {
                for p in maybe {
                    game.board.set(p.row, p.column, 'O');
                }
                continue
            }
            //
            // for p in maybe {
            //     game.board.set(p.row, p.column, 'I');
            // }
            //
        }
    }

    let mut result = 0;
    for row in 0..game.board.num_rows() {
        for column in 0..game.board.num_columns() {
            let item = game.board.get(row, column).unwrap();
            if *item == 'O' {
                result += 1;
            }
        }
    }

    result = game.board.num_rows() * game.board.num_columns() - result - longest.len();
    Ok(result as u32)
}

fn leakage(board: &mut Array2D<char>, p1: &Vec<Pipe>, maybe: &mut Vec<Pipe>, row: usize, column: usize) -> bool {
    if p1.contains(&Pipe { row, column }) {
        return false
    }

    if maybe.contains(&Pipe { row, column }) {
        return false
    }

    if *board.get(row, column).unwrap() == 'O' {
        return true
    }

    if *board.get(row, column).unwrap() == 'I' {
        return false
    }

    maybe.push(Pipe { row, column });

    if row == 0 || row == board.num_rows() - 1 {
        return true
    }

    if column == 0 || column == board.num_columns() - 1 {
        return true
    }

    let rm = leakage(board, p1, maybe, row - 1, column);
    if rm {
        return true
    }

    let rp = leakage(board, p1, maybe, row + 1, column);
    if rp {
        return true
    }

    let cm = leakage(board, p1, maybe, row, column - 1);
    if cm {
        return true
    }

    let cp = leakage(board, p1, maybe, row, column + 1);
    if cp {
        return true
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

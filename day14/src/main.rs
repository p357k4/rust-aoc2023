mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};

struct Input {
    tile: Array2D<char>,
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let rows = reader.lines()
        .map(|v| v.unwrap().chars().collect_vec()).collect_vec();

    let tile = Array2D::from_rows(&rows).unwrap();
    Ok(Input { tile })
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = load(path)?;

    let mut result = 0;
    let num_rows = input.tile.num_rows();
    let num_columns = input.tile.num_columns();

    for row in 0..num_rows {
        for column in 0..num_columns {
            let element = input.tile.get(row,column).unwrap();
            if *element == 'O' {
                slide(&mut input.tile, row, column);
            }
        }
    }

    for row in 0..num_rows {
        for column in 0..num_columns {
            let element = input.tile.get(row,column).unwrap();
            if *element == 'O' {
                result += num_rows - row;
            }
        }
    }

    Ok(result)
}

fn slide(tile: &mut Array2D<char>, row: usize, column: usize) {
    if row == 0 {
        return
    }

    let c = tile.get(row - 1, column).unwrap();

    if *c == '.' {
        tile.set(row-1, column, 'O');
        tile.set(row, column, '.');
        slide(tile, row - 1, column);
    }
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = load(path)?;

    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

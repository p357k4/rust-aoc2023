mod main_test;

use std::collections::HashMap;
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
            let element = input.tile.get(row, column).unwrap();
            if *element == 'O' {
                slide_north(&mut input.tile, row, column);
            }
        }
    }

    for row in 0..num_rows {
        for column in 0..num_columns {
            let element = input.tile.get(row, column).unwrap();
            if *element == 'O' {
                result += num_rows - row;
            }
        }
    }

    Ok(result)
}

fn slide_north(tile: &mut Array2D<char>, row: usize, column: usize) {
    if row == 0 {
        return;
    }

    let c = tile.get(row - 1, column).unwrap();

    if *c == '.' {
        tile.set(row - 1, column, 'O');
        tile.set(row, column, '.');
        slide_north(tile, row - 1, column);
    }
}


fn slide_south(tile: &mut Array2D<char>, row: usize, column: usize) {
    if row == tile.num_rows() - 1 {
        return;
    }

    let next = row + 1;
    let c = tile.get(next, column).unwrap();

    if *c == '.' {
        tile.set(next, column, 'O');
        tile.set(row, column, '.');
        slide_south(tile, next, column);
    }
}

fn slide_east(tile: &mut Array2D<char>, row: usize, column: usize) {
    if column == tile.num_columns() - 1 {
        return;
    }

    let next = column + 1;
    let c = tile.get(row, next).unwrap();

    if *c == '.' {
        tile.set(row, next, 'O');
        tile.set(row, column, '.');
        slide_east(tile, row, next);
    }
}

fn slide_west(tile: &mut Array2D<char>, row: usize, column: usize) {
    if column == 0 {
        return;
    }

    let next = column - 1;
    let c = tile.get(row, next).unwrap();

    if *c == '.' {
        tile.set(row, next, 'O');
        tile.set(row, column, '.');
        slide_west(tile, row, next);
    }
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = load(path)?;

    let num_rows = input.tile.num_rows();
    let num_columns = input.tile.num_columns();

    let mut result = 0;

    let mut cache = HashMap::new();
    let mut tiles: Vec<Array2D<char>> = vec![];
    let mut scores = vec![];

    for i in 0..1000000000 {
        for row in 0..num_rows {
            for column in 0..num_columns {
                let element = input.tile.get(row, column).unwrap();
                if *element == 'O' {
                    slide_north(&mut input.tile, row, column);
                }
            }
        }

        for row in 0..num_rows {
            for column in 0..num_columns {
                let element = input.tile.get(row, column).unwrap();
                if *element == 'O' {
                    slide_west(&mut input.tile, row, column);
                }
            }
        }

        for row in (0..num_rows).rev() {
            for column in 0..num_columns {
                let element = input.tile.get(row, column).unwrap();
                if *element == 'O' {
                    slide_south(&mut input.tile, row, column);
                }
            }
        }

        for row in 0..num_rows {
            for column in (0..num_columns).rev() {
                let element = input.tile.get(row, column).unwrap();
                if *element == 'O' {
                    slide_east(&mut input.tile, row, column);
                }
            }
        }

        for (cycle, ct) in tiles.iter().enumerate() {
            if ct.eq(&input.tile) {
                println!("possible cycle at {i} {cycle}");
                let sub = (1000000000 - i) % (i - cycle);
                let g = score(tiles.get(cycle + sub - 1).unwrap());
                println!("{g}");
                return Ok(g);
            }
        }

        result = score(&input.tile);
        println!("{result}");

        tiles.push(input.tile.clone());
        scores.push(result);

        if !cache.contains_key(&input.tile) {
            cache.insert(input.tile.clone(), result);
        }
    }


    Ok(result)
}

fn score(tile: &Array2D<char>) -> usize {
    let num_rows = tile.num_rows();

    let num_columns = tile.num_columns();

    let mut result = 0;

    for row in 0..num_rows {
        for column in 0..num_columns {
            let element = tile.get(row, column).unwrap();
            if *element == 'O' {
                result += num_rows - row;
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

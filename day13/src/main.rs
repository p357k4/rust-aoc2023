mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};


struct Block {
    rows: Vec<String>,
}

struct Input {
    blocks: Vec<Block>,
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let blocks_content = content.split("\n\n").collect_vec();

    let blocks = blocks_content.iter()
        .map(|&block_section| {
            let rows = block_section
                .split('\n')
                .map(|v| v.to_string())
                .collect_vec();
            Block { rows }
        }
        )
        .collect_vec();

    Ok(Input { blocks })
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;
    let mut vertical_sum = 0;
    let mut horizontal_sum = 0;

    for block in input.blocks {
        for shift in 1..block.rows.first().map(|v| v.len()).unwrap_or(0) {
            let equal = block.rows.iter().all(|v| v.bytes().take(shift).rev().zip(v.bytes().skip(shift)).all(|(left, right)| left == right));
            if equal {
                vertical_sum += shift;
            }
        }

        for shift in 1..block.rows.len() {
            let equal = block.rows.iter().take(shift).rev().zip(block.rows.iter().skip(shift)).all(|(left, right)| left == right);
            if equal {
                horizontal_sum += shift;
            }
        }
    }

    let result = horizontal_sum * 100 + vertical_sum;

    Ok(result)
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;
    let mut vertical_sum = 0;
    let mut horizontal_sum = 0;

    for block in input.blocks {
        for shift in 1..block.rows.first().map(|v| v.len()).unwrap_or(0) {
            let c: usize = block.rows
                .iter()
                .map(|v| {
                    v.bytes()
                        .take(shift)
                        .rev()
                        .zip(v.bytes().skip(shift))
                        .filter(|(left, right)| {
                            left != right
                        })
                        .count()
                })
                .sum();
            if c == 1 {
                vertical_sum += shift;
                break
            }
        }

        for shift in 1..block.rows.len() {
            let c: usize = block.rows
                .iter()
                .take(shift)
                .rev()
                .zip(block.rows.iter().skip(shift))
                .map(|(left, right)| {
                    left.chars().zip(right.chars())
                        .filter(|(left, right)| {
                            left != right
                        })
                        .count()
                })
                .sum();
            if c == 1 {
                horizontal_sum += shift;
                break
            }
        }
    }

    let result = horizontal_sum * 100 + vertical_sum;

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

mod main_test;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use array2d::Array2D;
use itertools::{Itertools};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;


#[derive(Clone, Eq, PartialEq, Hash)]
struct Block {
    x0: u32,
    y0: u32,
    z0: u32,
    x1: u32,
    y1: u32,
    z1: u32,
}

struct Input {
    blocks: Vec<Block>,
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;

    tuple((separated_list1(tag(","), u32), tag("~"), separated_list1(tag(","), u32)))(input)
        .map(|(leftover, (start, _, end))| {
            (leftover, Block {
                x0: start[0],
                y0: start[1],
                z0: start[2],
                x1: end[0],
                y1: end[1],
                z1: end[2],
            })
        })
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let blocks = reader.lines()
        .flatten()
        .map(|line| parse_block(&line).unwrap().1)
        .collect_vec();

    Ok(Input { blocks })
}

fn clip(knife: &Block, butter: &Block) -> Block {
    let mut q = butter.clone();

    if q.x0 <= knife.x0 && knife.x0 <= q.x1 {
        q.x0 = knife.x0;
    }

    if q.x0 <= knife.x1 && knife.x1 <= q.x1 {
        q.x1 = knife.x1;
    }

    if q.y0 <= knife.y0 && knife.y0 <= q.y1 {
        q.y0 = knife.y0;
    }

    if q.y0 <= knife.y1 && knife.y1 <= q.y1 {
        q.y1 = knife.y1;
    }

    if q.z0 <= knife.z0 && knife.z0 <= q.z1 {
        q.z0 = knife.z0;
    }

    if q.z0 <= knife.z1 && knife.z1 <= q.z1 {
        q.z1 = knife.z1;
    }

    q.clone()
}

fn is_inside(left: &Block, right: &Block) -> bool {
    (right.x0 <= left.x0 && left.x1 <= right.x1) && (right.y0 <= left.y0 && left.y1 <= right.y1)
}

fn part1(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = load(path)?;

    let mut supported = vec![];

    let sorted = input.blocks.iter().sorted_by_key(|block| 1_000_000 - block.z0).collect_vec();

    let length = sorted.len();
    for i in 0..length {
        let mut block = sorted[i].clone();

        for j in i + 1..length {
            let other = sorted[j];
            let clipped = &clip(&block, other);
            if !is_inside(clipped, &block) {
                let dz = block.z0 - other.z0;
                block.z0 -= dz;
                block.z1 -= dz;
            }
        }

        supported.push(block);
    }

    let blocks = supported.iter().sorted_by_key(|block| 1_000_000 - block.z0).collect_vec();

    let result = blocks.len();

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

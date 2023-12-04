mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;
use std::str::FromStr;
use itertools::Itertools;

struct Draw {
    winning: Vec<u32>,
    have: Vec<u32>,
}

struct Game {
    draws: Vec<Draw>,
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let draws = reader.lines().map(|line_result| {
        line_result.map(|line| {
            let split0 = line.split(" ").filter(|&x| !x.is_empty()).collect_vec();
            let mut i = 2;
            let mut winning = vec![];
            let mut have = vec![];

            loop {
                let token = *split0.get(i).unwrap();
                i += 1;
                if token == "|" {
                    break;
                }

                let number = u32::from_str(token).unwrap();
                winning.push(number);
            }

            loop {
                if i == split0.len() {
                    break;
                }

                let token = split0.get(i).unwrap();
                i += 1;

                let number = u32::from_str(token).unwrap();
                have.push(number);
            }

            Draw { winning, have }
        })
    }).fold(vec![], |mut acc, b| {
        if let Ok(draw) = b {
            acc.push(draw);
        }
        acc
    });

    Ok(Game { draws })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let score = |draw: &Draw| {
        draw.have.iter().filter(|h| draw.winning.contains(h)).count() as u32
    };

    let hits = game.draws.iter().map(score).collect_vec();

    let result = hits.iter().map(|h| if *h == 0 {
        0
    } else {
        1u32.checked_shl(*h - 1).unwrap()
    }
    ).collect_vec();

    let sum = result.iter().sum();
    Ok(sum)
}

fn part2(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let score = |draw: &Draw| {
        draw.have.iter().filter(|h| draw.winning.contains(h)).count() as u32
    };

    let mut scores : Vec<u32> = vec![0; game.draws.len()];
    for enumerate in game.draws.iter().enumerate() {
        let score = score(enumerate.1);
        scores[enumerate.0] += 1;
        for i in 0..score {
            scores[enumerate.0 + i as usize + 1] += scores[enumerate.0];
        }
    }

    let sum = scores.iter().sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

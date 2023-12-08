mod main_test;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use itertools::{Itertools};
use nom::character::complete;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric0, line_ending, space1};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;

struct Hand {
    cards: String,
    bid: u64,
}

#[derive(Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

struct Game {
    steps: String,
    nodes: HashMap<String, Node>,
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    tuple((parse_string, tag(" = ("), parse_string, tag(", "), parse_string, tag(")"), opt(line_ending)))(input)
        .map(|(leftover, (node_name, _, left_node, _, right_node, _, _))| {
            (leftover, Node { name: node_name.to_string(), left: left_node.to_string(), right: right_node.to_string() })
        }
        )
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    let (body, (steps, _, _, ns)) = tuple((parse_string, line_ending, line_ending, many0(parse_node)))(content.as_str()).unwrap();

    let nodes = HashMap::from_iter(ns.iter().map(|n| (n.name.clone(), n.clone())));

    Ok(Game { steps: steps.to_string(), nodes })
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let mut name = "AAA".to_string();
    for (counter, step) in game.steps.chars().cycle().enumerate() {
        if name == "ZZZ" {
            return Ok(counter as u64)
        }
        let node = game.nodes.get(&name).unwrap();
        if step == 'L' {
            name = node.left.clone();
        } else if step == 'R' {
            name = node.right.clone();
        }
    }
    Ok(0)
}

fn steps(game: &Game, start: &String) -> u64 {
    let mut name = start.clone();

    for (counter, step) in game.steps.chars().cycle().enumerate() {
        if name.ends_with("Z") {
            return counter as u64
        }
        let node = game.nodes.get(&name).unwrap();
        if step == 'L' {
            name = node.left.clone();
        } else if step == 'R' {
            name = node.right.clone();
        }
    }
    0
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let names = game.nodes.keys().filter(|&x| x.ends_with("A")).collect_vec();

    let steps = names.iter().map(|name| steps(&game,name)).collect_vec();

    let mut h = 1;
    for x in steps {
        h = (h * x) / gcd(h, x)
    }
    Ok(h)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

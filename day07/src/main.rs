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
use nom::character::complete::alphanumeric0;
use nom::multi::many0;
use nom::sequence::tuple;

struct Hand {
    cards: String,
    bid: u64,
}

struct Game {
    observations: Vec<Hand>,
}

fn calculate_hand(cards: &String) -> Vec<usize> {
    let strengths = cards
        .chars()
        .counts()
        .values()
        .copied()
        .sorted_unstable()
        .rev()
        .collect_vec();

    strengths
}

fn calculate_hand_joker(cards: &String) -> Vec<usize> {
    let jokers = cards.chars().filter(|&c| c == 'J').count();

    let mut strengths = cards
        .chars()
        .filter(|&c| c != 'J')
        .counts()
        .values()
        .copied()
        .sorted_unstable()
        .rev()
        .collect_vec();

    if strengths.len() > 0 {
        strengths[0] += jokers;
    } else {
        strengths.push(jokers);
    }

    strengths
}

fn compare_cards_joker(left: &String, right: &String) -> Ordering {
    let scores = HashMap::from([
        ('A', 1),
        ('K', 2),
        ('Q', 3),
        ('T', 4),
        ('9', 5),
        ('8', 6),
        ('7', 7),
        ('6', 8),
        ('5', 9),
        ('4', 10),
        ('3', 11),
        ('2', 12),
        ('J', 13),
    ]);

    left.chars().zip(right.chars())
        .map(|(l, r)| scores[&l].cmp(&scores[&r]))
        .find(|&o| o != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

fn compare_cards(left: &String, right: &String) -> Ordering {
    let scores = HashMap::from([
        ('A', 1),
        ('K', 2),
        ('Q', 3),
        ('J', 4),
        ('T', 5),
        ('9', 6),
        ('8', 7),
        ('7', 8),
        ('6', 9),
        ('5', 10),
        ('4', 11),
        ('3', 12),
        ('2', 13),
    ]);

    left.chars().zip(right.chars())
        .map(|(l, r)| scores[&l].cmp(&scores[&r]))
        .find(|&o| o != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

fn left_won(left: &Hand, right: &Hand, calculate_hand: fn(&String) -> Vec<usize>, compare_cards: fn(&String, &String) -> Ordering) -> bool {
    let left_strength = calculate_hand(&left.cards);
    let right_strength = calculate_hand(&right.cards);

    let hand_ordering = left_strength.iter().cmp(right_strength.iter());
    match hand_ordering {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => {
            let card_ordering = compare_cards(&left.cards, &right.cards);

            match card_ordering {
                Ordering::Less => true,
                Ordering::Greater => false,
                Ordering::Equal => true,
            }
        },
    }
}

fn cards_parser(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

fn load(path: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let observations = reader.lines().map(|line_result| {
        let line = line_result.unwrap();
        let (leftover, (cards, _, bid)) = tuple((cards_parser, complete::space1, complete::u64))(line.as_str()).unwrap();
        Hand { cards: cards.to_string(), bid }
    })
        .collect_vec();

    Ok(Game { observations })
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result = game.observations.iter()
        .map(|left| {
            let rank = game.observations.iter()
                .filter(|right| left_won(&left, &right, calculate_hand, compare_cards))
                .count() as u64;

            left.bid * rank
        }
        ).sum();

    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let game = load(path)?;

    let result: u64 = game.observations.iter()
        .map(|left| {
            let rank = game.observations.iter()
                .filter(|right| left_won(&left, &right, calculate_hand_joker, compare_cards_joker))
                .count() as u64;

            left.bid * rank
        }
        ).sum();

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

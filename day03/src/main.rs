mod main_test;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use array2d::Array2D;
use itertools::Itertools;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct StarDetail {
    star: Position,
    number: Position,
}

#[derive(Clone)]
enum Element {
    Digit(u32),
    Symbol(char),
    Period,
}

struct Board {
    b: Array2D<Element>,
}

impl Board {
    fn is_symbol(&self, row: i32, column: i32) -> bool {
        if row < 0 || column < 0 {
            false
        } else {
            matches!(self.b.get(row as usize, column as usize), Some(Element::Symbol(_)))
        }
    }

    fn is_star(&self, row: i32, column: i32) -> bool {
        if row < 0 || column < 0 {
            false
        } else if let Some(Element::Symbol(x)) = self.b.get(row as usize, column as usize) {
            *x == '*'
        } else {
            false
        }
    }

    fn check_adjacent(&self, row: usize, column: usize) -> bool {
        let r = row as i32;
        let c = column as i32;

        self.is_symbol(r, c - 1)
            || self.is_symbol(r, c + 1)
            || self.is_symbol(r - 1, c - 1)
            || self.is_symbol(r - 1, c)
            || self.is_symbol(r - 1, c + 1)
            || self.is_symbol(r + 1, c - 1)
            || self.is_symbol(r + 1, c)
            || self.is_symbol(r + 1, c + 1)
    }

    fn star_position(&self, row: usize, column: usize) -> Vec<Position> {
        let r = row as i32;
        let c = column as i32;
        let mut p = vec![];

        if self.is_star(r, c - 1) {
            p.push(Position { row: row, column: column - 1 })
        }
        if self.is_star(r, c + 1) {
            p.push(Position { row: row, column: column + 1 })
        }
        if self.is_star(r - 1, c - 1) {
            p.push(Position { row: row - 1, column: column - 1 })
        }
        if self.is_star(r - 1, c) {
            p.push(Position { row: row - 1, column: column })
        }
        if self.is_star(r - 1, c + 1) {
            p.push(Position { row: row - 1, column: column + 1 })
        }
        if self.is_star(r + 1, c - 1) {
            p.push(Position { row: row + 1, column: column - 1 })
        }
        if self.is_star(r + 1, c) {
            p.push(Position { row: row + 1, column: column })
        }
        if self.is_star(r + 1, c + 1) {
            p.push(Position { row: row + 1, column: column + 1 })
        }

        p
    }
}

fn load(path: &str) -> Result<Board, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let rows = reader.lines().map(|line_result| {
        let line_str = line_result.unwrap();
        line_str.chars().map(|x| {
            if x.is_ascii_digit() {
                Element::Digit(x.to_digit(10).unwrap())
            } else if x == '.' {
                Element::Period
            } else {
                Element::Symbol(x)
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let b = Array2D::from_rows(&rows).unwrap();

    Ok(Board { b })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let board = load(path)?;
    let mut part_numbers: Vec<u32> = vec![];

    for i in 0..board.b.num_rows() {
        let mut part_number = 0;
        let mut is_part_number = false;
        for j in 0..board.b.num_columns() {
            let element = board.b.get(i, j).unwrap();
            match element {
                Element::Digit(x) => {
                    if board.check_adjacent(i, j) {
                        is_part_number = true;
                    }
                    part_number = part_number * 10 + x;
                }
                _ => {
                    if part_number != 0 && is_part_number {
                        part_numbers.push(part_number)
                    }
                    is_part_number = false;
                    part_number = 0;
                }
            }
        }

        if part_number != 0 && is_part_number {
            part_numbers.push(part_number)
        }
        is_part_number = false;
        part_number = 0;
    }

    let result = part_numbers.iter().sum();
    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let board = load(path)?;
    let mut numbers: HashMap<Position, u64> = HashMap::new();
    let mut star_numbers: HashSet<StarDetail> = HashSet::new();

    let mut number_row = 0;
    let mut number_column = 0;

    for i in 0..board.b.num_rows() {
        let mut part_number = 0u64;
        for j in 0..board.b.num_columns() {
            let element = board.b.get(i, j).unwrap();
            match element {
                Element::Digit(x) => {
                    if part_number == 0 {
                        number_row = i;
                        number_column = j;
                    }
                    let star_positions = board.star_position(i, j);
                    for star_position in star_positions {
                        star_numbers.insert(StarDetail {
                            star: star_position,
                            number: Position { row: number_row, column: number_column },
                        });
                    }
                    part_number = part_number * 10 + *x as u64;
                }
                _ => {
                    if part_number != 0 {
                        numbers.insert(Position { row: number_row, column: number_column }, part_number);
                    }
                    part_number = 0;
                }
            }
        }

        if part_number != 0 {
            numbers.insert(Position { row: number_row, column: number_column }, part_number);
        }
        part_number = 0;
    }

    let f = star_numbers.iter().into_group_map_by(|v| v.star);

    let g = f.iter().filter(|&v| v.1.len() > 1).collect_vec();

    let h = g.iter().map(|&v| v.1.iter().fold(1, |acc, y| acc * numbers.get(&y.number).unwrap())).collect_vec();
    let result = h.iter().sum();
    // let grouped = star_numbers.iter()
    //     .group_by(|&x| x.star)
    //     .into_iter()
    //     .map(|x| {
    //         let n = x.1.collect::<Vec<_>>();
    //         if n.len() == 2 {
    //             let v = n.iter().fold(1, |acc, y| acc * numbers.get(&y.number).unwrap());
    //             v
    //         } else {
    //             0
    //         }
    //     })
    //     .filter(|x| *x != 0)
    //     .collect::<Vec<_>>();
    //
    // let result = grouped.iter().sum();
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

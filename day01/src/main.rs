use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let first = line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last = line.chars().rfind(|x| x.is_ascii_digit()).unwrap();

        let a = first.to_digit(10).unwrap();
        let b = last.to_digit(10).unwrap();

        let value = a * 10 + b;
        sum += value;
    }

    println!("{}", sum);
    Ok(())
}

fn part2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    struct PatternItem<'a> {
        pattern: &'a str,
        value: i32,
    }
    let patterns = [
        PatternItem { pattern: "zero", value: 0 },
        PatternItem { pattern: "one", value: 1 },
        PatternItem { pattern: "two", value: 2 },
        PatternItem { pattern: "three", value: 3 },
        PatternItem { pattern: "four", value: 4 },
        PatternItem { pattern: "five", value: 5 },
        PatternItem { pattern: "six", value: 6 },
        PatternItem { pattern: "seven", value: 7 },
        PatternItem { pattern: "eight", value: 8 },
        PatternItem { pattern: "nine", value: 9 },
        PatternItem { pattern: "0", value: 0 },
        PatternItem { pattern: "1", value: 1 },
        PatternItem { pattern: "2", value: 2 },
        PatternItem { pattern: "3", value: 3 },
        PatternItem { pattern: "4", value: 4 },
        PatternItem { pattern: "5", value: 5 },
        PatternItem { pattern: "6", value: 6 },
        PatternItem { pattern: "7", value: 7 },
        PatternItem { pattern: "8", value: 8 },
        PatternItem { pattern: "9", value: 9 },
    ];

    let mut sum = 0;
    struct FoundItem {
        index: usize,
        value: i32,
    }

    for line_result in reader.lines() {
        let line = line_result?;
        let mut found = vec![];
        for p in patterns.iter() {
            let index_option = line.find(p.pattern);
            if let Some(index) = index_option {
                found.push(FoundItem { index, value: p.value });
            }
            let rindex_option = line.rfind(p.pattern);
            if let Some(rindex) = rindex_option {
                found.push(FoundItem { index: rindex, value: p.value });
            }
        }
        found.sort_by(|a, b| a.index.cmp(&b.index));

        let a = found.first().unwrap().value;
        let b = found.last().unwrap().value;

        let value = a * 10 + b;
        sum += value;
    }

    println!("{}", sum);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1("example/day01_part1_example.txt")?;
    part1("data/day01_part1.txt")?;
    part2("example/day01_part2_example.txt")?;
    part2("data/day01_part2.txt")?;

    Ok(())
}

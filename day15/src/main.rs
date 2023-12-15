mod main_test;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};

struct Input {
    sequences: Vec<Vec<String>>,
}

// rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let sequences = reader.lines()
        .map(|v| {
            let line = v.unwrap();
            line.split(",").map(|v| v.to_string()).collect_vec()
        }).collect_vec();

    Ok(Input { sequences })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let s = input.sequences.iter().map(|seq| {
        let r: u32 = seq.iter().map(hash).sum();
        r
    })
        .collect_vec();
    let result = s.iter().sum();

    Ok(result)
}

fn hash(v: &String) -> u32 {
    let mut acc = 0;
    for b in v.as_bytes() {
        acc += (*b as u32);
        acc *= 17;
        acc %= 256;
    }
    acc
}

#[derive(Clone)]
struct Len {
    label: String,
    focal: usize,
}

#[derive(Clone)]
struct LensBox {
    lens: Vec<Len>,
}

fn part2(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let sequence = input.sequences.first().unwrap();

    let mut boxes: Vec<LensBox> = vec![];

    for i in 0..256 {
        boxes.push(LensBox { lens: vec![] })
    }

    for seq in sequence {
        if seq.ends_with('-') {
            let (label, _) = seq.split_once('-').unwrap();
            let n = hash(&label.to_string());

            let lens_box = boxes.get_mut(n as usize).unwrap();
            if let Some(p) = lens_box.lens.iter().position(|len| len.label == label) {
                lens_box.lens.remove(p);
            }
        } else {
            let (label, focal_length) = seq.split_once('=').unwrap();
            let n = hash(&label.to_string());
            let f = usize::from_str_radix(focal_length, 10).unwrap();
            let len = Len { label: label.to_string(), focal: f };
            if boxes.get_mut(n as usize).is_none() {
                println!("help!")
            }
            let lens_box = boxes.get_mut(n as usize).unwrap();
            if let Some(p) = lens_box.lens.iter().position(|len| len.label == label) {
                lens_box.lens.as_mut_slice()[p] = len;
            } else {
                lens_box.lens.push(len);
            }
        }
    }

    let mut result = 0;
    for (n, lens_box) in boxes.iter().enumerate() {
        for (l, len) in lens_box.lens.iter().enumerate() {
            result += (n + 1) * (l + 1) * len.focal;
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

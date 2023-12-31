use std::collections::{HashMap, vec_deque, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

mod main_test;

#[derive(Clone, Eq, PartialEq, Debug)]
enum ModuleType {
    FlipFlop,
    Broadcaster,
    Conjunction,
}

struct Module {
    inputs: Vec<String>,
    outputs: Vec<String>,
    module_type: ModuleType,
}

struct Output {
    value: bool,
}

struct Input {
    modules: HashMap<String, Module>,
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

fn parse_broadcaster(input: &str) -> IResult<&str, (&str, ModuleType)> {
    alphanumeric0(input)
        .map(|(leftover, (name))| {
            (leftover, (name, ModuleType::Broadcaster))
        })
}

fn parse_flipflop(input: &str) -> IResult<&str, (&str, ModuleType)> {
    tuple((tag("%"), alphanumeric0))(input)
        .map(|(leftover, (_, name))| {
            (leftover, (name, ModuleType::FlipFlop))
        })
}

fn parse_conjunction(input: &str) -> IResult<&str, (&str, ModuleType)> {
    tuple((tag("&"), alphanumeric0))(input)
        .map(|(leftover, (_, name))| {
            (leftover, (name, ModuleType::Conjunction))
        })
}

fn parse_line(input: &str) -> IResult<&str, (String, ModuleType, Vec<String>)> {
    tuple((alt((parse_flipflop, parse_conjunction, parse_broadcaster)), tag(" -> "), separated_list0(tag(", "), alphanumeric0)))(input)
        .map(|(leftover, ((name, module), _, names))| {
            (leftover, (name.to_string(), module, names.iter().map(|v| v.to_string()).collect_vec()))
        })
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    reader.read_to_string(&mut content);

    let ms = content.split("\n")
        .flat_map(|line| {
            parse_line(line)
        })
        .collect_vec();

    let output_names: HashMap<String, Vec<String>> = HashMap::from_iter(
        ms.iter()
            .map(|(_, (name, module, names))| {
                (name.to_string(), names.clone())
            })
    );

    let mut modules = HashMap::from_iter(
        ms.iter()
            .map(|(_, (name, module_type, names))| {
                let inputs = output_names.iter()
                    .filter(|(wire_name, wire_names)| wire_names.contains(name))
                    .map(|(wire_name, wire_names)| wire_name.to_string())
                    .collect_vec();

                let module = Module {
                    inputs,
                    outputs: names.clone(),
                    module_type: module_type.clone(),
                };

                (name.to_string(), module)
            })
    );

    modules.insert("rx".to_string(), Module {
        inputs: vec![],
        outputs: vec![],
        module_type: ModuleType::Broadcaster,
    });

    Ok(Input { modules })
}

fn update(input: &Input, outputs: &mut HashMap<String, Output>, name: &String, pulse: bool, low: &mut u64, high: &mut u64, gu: &mut VecDeque<(String, bool)>) {
    if pulse {
        *high += 1;
    } else {
        *low += 1;
    }

    let Some(module) = input.modules.get(name) else { return; };

    match module.module_type {
        ModuleType::FlipFlop => {
            let output = outputs.get_mut(name).unwrap();
            if pulse == false {
                output.value = !output.value;
                for output_name in &module.outputs {
                    gu.push_back((output_name.clone(), output.value))
                }
            }
        }
        ModuleType::Broadcaster => {
            let output = outputs.get_mut(name).unwrap();
            output.value = pulse;
            for output_name in &module.outputs {
                gu.push_back((output_name.clone(), output.value))
            }
        }
        ModuleType::Conjunction => {
            let all = module.inputs.iter().all(|name| outputs.get(name).unwrap().value);
            let output = outputs.get_mut(name).unwrap();
            output.value = !all;
            for output_name in &module.outputs {
                gu.push_back((output_name.clone(), output.value))
            }
        }
    }
}

fn energize(input: &Input, outputs: &mut HashMap<String, Output>, low: &mut u64, high: &mut u64, gu: &mut VecDeque<(String, bool)>) {
    while let Some((name, pulse)) = gu.pop_front() {
        update(input, outputs, &name, pulse, low, high, gu);
    }
}

fn part1(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut outputs = HashMap::from_iter(
        input.modules
            .keys()
            .map(|name| (name.to_string(), Output { value: false }))
            .collect_vec()
    );

    let mut low = 0;
    let mut high = 0;

    for i in 0..1000 {
        let mut updates: VecDeque<(String, bool)> = VecDeque::from([("broadcaster".into(), false)]);
        energize(&input, &mut outputs, &mut low, &mut high, &mut updates);
    }

    let result = low * high;
    Ok(result)
}

fn part2(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut outputs = HashMap::from_iter(
        input.modules.keys().map(|name| (name.to_string(), Output { value: false }))
            .collect_vec()
    );

    outputs.get_mut(&"rx".to_string()).unwrap().value = true;

    let mut low = 0;
    let mut high = 0;
    let mut result = 0;

    let gates = input.modules.iter().filter(|(name, module)| module.module_type == ModuleType::Conjunction).collect_vec();

    for i in 0..100000000 {
        let mut updates: VecDeque<(String, bool)> = VecDeque::from([("broadcaster".into(), false)]);
        energize(&input, &mut outputs, &mut low, &mut high, &mut updates);
        if outputs.get(&"rx".to_string()).unwrap().value == false {
            result = i;
            break;
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::{Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1};
use nom::combinator::opt;
use nom::error::Error;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

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

    let modules = HashMap::from_iter(
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
                }
                    ;
                (name.to_string(), module)
            })
    );

    Ok(Input { modules, })
}


fn energize(input: &Input, outputs: &mut HashMap<String, Output>, name: &String, pulse: bool) {
    let module = input.modules.get(name).unwrap();

    match module.module_type {
        ModuleType::FlipFlop => {
            let mut output = outputs.get_mut(name).unwrap();
            if pulse == false {
                output.value = !output.value;
                let value = output.value;
                module.outputs.iter().for_each(|output_name| energize(input, outputs, output_name, value))
            }
        }
        ModuleType::Broadcaster => {
            let mut output = outputs.get_mut(name).unwrap();
            output.value = pulse;
            let value = output.value;
            module.outputs.iter().for_each(|output_name| energize(input, outputs, output_name, value))
        }
        ModuleType::Conjunction => {
            let all = module.inputs.iter().all(|name| outputs.get(name).unwrap().value);
            let mut output = outputs.get_mut(name).unwrap();
            output.value = all;
            let value = output.value;
            module.outputs.iter().for_each(|output_name| energize(input, outputs, output_name, value))
        }
    }
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut outputs = HashMap::from_iter(
        input.modules.keys().map(|name| (name.to_string(), Output { value: false }))
            .collect_vec()
    );

    energize(&input, &mut outputs, &"broadcaster".into(), false);

    let result = 0;
    Ok(result)
}

fn part2(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let result = 0;
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

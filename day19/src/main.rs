mod main_test;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use itertools::{Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1};
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

#[derive(Clone, Eq, PartialEq)]
struct Part {
    v: HashMap<String, u32>,
}

#[derive(Clone, Copy)]
struct Range {
    low: u32,
    high: u32,
}

#[derive(Clone)]
struct RangePart {
    v: HashMap<String, Range>,
}

struct Condition {
    category: String,
    operator: String,
    value: u32,
}

struct Rule {
    condition: Option<Condition>,
    to: String,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Input {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

fn parse_attribute(input: &str) -> IResult<&str, (&str, u32)> {
    let r = tuple((alphanumeric0, tag("="), complete::u32))(input);

    r.map(|(leftover, (name, _, value))| {
        (leftover, (name, value))
    })
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let r = tuple((tag("{"), separated_list0(tag(","), parse_attribute), tag("}")))(input);

    r.map(|(leftover, (_, vec, _))| {
        let mut v = HashMap::new();

        for t in vec {
            v.insert(t.0.to_string(), t.1);
        }

        (leftover, Part { v })
    })
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let r = tuple((alpha1, alt((tag(">"), tag("<"))), complete::u32))(input);

    r.map(|(leftover, (cat, op, v))| {
        (
            leftover,
            Condition {
                category: cat.to_string(),
                operator: op.to_string(),
                value: v,
            }
        )
    })
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    tuple((opt(tuple((parse_condition, tag(":")))), alphanumeric1))(input)
        .map(|(leftover, (opt, to))| {
            (leftover, Rule { condition: opt.map(|v| v.0), to: to.to_string() })
        })
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    tuple((alphanumeric1, tag("{"), separated_list0(tag(","), parse_rule), tag("}")))(input)
        .map(|(leftover, (name, _, rules, _))| {
            (leftover, Workflow { name: name.to_string(), rules: rules })
        })
}

fn load(path: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content);

    let Some((workflows_str, parts_str)) = content.split_once("\n\n") else { todo!() };

    let workflows = workflows_str.split("\n").map(|line|
        {
            let Ok((leftover, workflow)) = parse_workflow(line) else { todo!() };

            workflow
        }).collect_vec();

    let parts = parts_str.split("\n").map(|line|
        {
            let Ok((leftover, part)) = parse_part(line) else { todo!() };

            part
        }).collect_vec();

    Ok(Input { workflows, parts })
}

fn part1(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let mut result = 0;
    for part in &input.parts {
        let accepted = analyze(&input.workflows, part, &"in".into());
        if accepted {
            let delta: u32= part.v.values().sum();
            result += delta;
        }
    }

    Ok(result)
}

fn analyze(workflows: &Vec<Workflow>, part: &Part, name: &String) -> bool {
    let Some(workflow) = workflows.iter().find_or_first(|w| w.name == *name) else { todo!() };

    let rule_option = workflow.rules.iter().find(|rule| {
        match &rule.condition {
            Some(condition) if condition.operator == ">" && part.v[&condition.category] > condition.value => {
                true
            },

            Some(condition) if condition.operator == "<" && part.v[&condition.category] < condition.value => {
                true
            },

            Some(_) => {
                false
            },

            None => {true},
        }
    });

    match rule_option {
        Some(rule) if rule.to == "A" => {
            true
        }
        Some(rule) if rule.to == "R" => {
            false
        }
        Some(rule) => {
            analyze(workflows, part, &rule.to)
        }
        None => {
            todo!()
        }
    }
}

fn part2(path: &str) -> Result<u128, Box<dyn std::error::Error>> {
    let input = load(path)?;

    let v = HashMap::from([
        ("x".to_string(), Range{ low: 1, high: 4000 }),
        ("m".to_string(), Range{ low: 1, high: 4000 }),
        ("a".to_string(), Range{ low: 1, high: 4000 }),
        ("s".to_string(), Range{ low: 1, high: 4000 }),
    ]);
    let part = RangePart{ v };

    let result = analyze2(&input.workflows, part, &"in".into());
    Ok(result)
}


fn analyze2(workflows: &Vec<Workflow>, part: RangePart, name: &String) -> u128 {
    if name == "A" {
        return part.v.values().fold(1u128, |acc, r| {
            acc * ((r.high - r.low + 1) as u128)
        })
    }

    if name == "R" {
        return 0
    }

    let Some(workflow) = workflows.iter().find_or_first(|w| w.name == *name) else { todo!() };

    for rule in &workflow.rules {
        match &rule.condition {
            Some(condition) if condition.operator == ">" => {
                if part.v[&condition.category].low > condition.value  {
                    // whole part meets condition
                    return analyze2(workflows, part, &rule.to)
                } else if part.v[&condition.category].high <= condition.value {
                    // whole part does not meet condition
                    ()
                } else {
                    let low = Range {high:condition.value, ..part.v[&condition.category]};
                    let high = Range {low:condition.value + 1, ..part.v[&condition.category]};

                    let mut low_part = part.clone();
                    let mut high_part = part.clone();
                    low_part.v.insert(condition.category.clone(), low);
                    high_part.v.insert(condition.category.clone(), high);

                    return analyze2(workflows, high_part, &rule.to) + analyze2(workflows, low_part, name)
                }
            },

            Some(condition) if condition.operator == "<"  => {
                if part.v[&condition.category].high < condition.value  {
                    // whole part meets condition
                    return analyze2(workflows, part, &rule.to)
                } else if part.v[&condition.category].low >= condition.value {
                    // whole part does not meet condition
                    ()
                } else {
                    let low = Range {high:condition.value - 1, ..part.v[&condition.category]};
                    let high = Range {low:condition.value, ..part.v[&condition.category]};

                    let mut low_part = part.clone();
                    let mut high_part = part.clone();
                    low_part.v.insert(condition.category.clone(), low);
                    high_part.v.insert(condition.category.clone(), high);

                    return analyze2(workflows, low_part, &rule.to) + analyze2(workflows, high_part, name)
                }
            },

            Some(_) => {
                todo!()
            },

            None => {
                return analyze2(workflows, part, &rule.to)
            },
        }
    }


    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

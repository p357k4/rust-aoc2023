mod main_test;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Color {
    Red,
    Green,
    Blue,
    Other,
}

struct GameDraw {
    color: Color,
    amount: i32,
}

struct GameSet {
    draws: Vec<GameDraw>,
}

struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

fn load_game(game_str: String) -> Result<Game, Box<dyn std::error::Error>> {
    let split0 = game_str.split(": ").collect::<Vec<_>>();
    let game_section_str = split0.first().ok_or("game section")?;
    let games_section_str = split0.last().ok_or("games section")?;
    let games_str = games_section_str.split("; ").collect::<Vec<_>>();

    let split1 = game_section_str.split(" ").collect::<Vec<_>>();
    let game_id_str = split1.last().ok_or("game id")?;
    let id = str::parse::<i32>(game_id_str)?;

    let sets = games_str.iter().map(|game_str| {
        let cubes_str = game_str.split(", ").collect::<Vec<_>>();
        let draws = cubes_str.iter().map(|cube| {
            let c_str = cube.split(" ").collect::<Vec<_>>();
            let amount_str = c_str.first().unwrap();
            let amount = str::parse::<i32>(amount_str).unwrap();

            let color_str = *c_str.last().unwrap();

            let color = match color_str {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                other => Color::Other,
            };

            GameDraw { amount, color: color }
        }).collect::<Vec<_>>();

        GameSet { draws }
    }).collect::<Vec<_>>();

    Ok(Game { sets, id })
}

fn load(path: &str) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let games = reader.lines().map(|game_result| {
        let game_str = game_result.unwrap();
        load_game(game_str).unwrap()
    }).collect::<Vec<_>>();

    Ok(games)
}

fn part1(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let games = load(path).unwrap();

    let result = games.iter().map(|game| {
        let nok = game.sets.iter().any(|set| {
            set.draws.iter().any(|draw| {
                match draw.color {
                    Color::Red => draw.amount > 12,
                    Color::Green => draw.amount > 13,
                    Color::Blue => draw.amount > 14,
                    Color::Other => true,
                }
            })
        });

        if nok {
            0
        } else {
            game.id
        }
    }).sum();

    Ok(result)
}

fn part2(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let games = load(path).unwrap();

    struct M {
        red: i32,
        green: i32,
        blue: i32,
    }

    let result = games.iter().map(|game| {
        let m = game.sets.iter().flat_map(|set| set.draws.iter())
            .fold(M { red: 0, green: 0, blue: 0 }, |acc, draw| {
                match draw.color {
                    Color::Red => M { red: max(acc.red, draw.amount), ..acc },
                    Color::Green => M { green: max(acc.green, draw.amount), ..acc },
                    Color::Blue => M { blue: max(acc.blue, draw.amount), ..acc },
                    Color::Other => acc,
                }
            });

        m.red * m.green * m.blue
    }).sum();

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1("example/day02_part1_example.txt")?;
    part1("data/day02_part1.txt")?;
    part2("example/day02_part1_example.txt")?;
    part2("data/day02_part1.txt")?;

    Ok(())
}

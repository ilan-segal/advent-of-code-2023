use std::cmp::max;
use std::str::FromStr;

use common;

enum Colour {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {
    message: String,
}

impl FromStr for Colour {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::RED),
            "green" => Ok(Self::GREEN),
            "blue" => Ok(Self::BLUE),
            _ => Result::Err(ParseError {
                message: format!("Invalid colour {}", s),
            }),
        }
    }
}

#[derive(Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn add(self: &mut Self, other: &Self) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }

    fn is_possible(self: &Self, real_amounts: &Self) -> bool {
        self.red <= real_amounts.red
            && self.blue <= real_amounts.blue
            && self.green <= real_amounts.green
    }

    fn maximum(self: &Self, other: &Self) -> Self {
        CubeSet {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn power(self: &Self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

impl Default for CubeSet {
    fn default() -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl FromStr for CubeSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, colour) = s.split_once(' ').ok_or(ParseError {
            message: format!("Failed to parse reveal {}", s),
        })?;
        let parsed_amount = amount
            .parse::<_>()
            .map_err(|_| ParseError {
                message: format!("Failed to parse reveal {}", s),
            })?;
        let parsed_color = Colour::from_str(colour)?;
        Ok(match parsed_color {
            Colour::RED => CubeSet {
                red: parsed_amount,
                green: 0,
                blue: 0,
            },
            Colour::GREEN => CubeSet {
                red: 0,
                green: parsed_amount,
                blue: 0,
            },
            Colour::BLUE => CubeSet {
                red: 0,
                green: 0,
                blue: parsed_amount,
            },
        })
    }
}

struct Game {
    id: u32,
    rounds: Vec<CubeSet>,
}

impl Game {
    fn is_possible(self: &Self, real_amounts: &CubeSet) -> bool {
        self.rounds
            .iter()
            .all(|reveal| reveal.is_possible(real_amounts))
    }

    fn get_minimum_set(self: &Self) -> CubeSet {
        self.rounds
            .iter()
            .map(|x| *x)
            .reduce(|acc, e| acc.maximum(&e))
            .unwrap()
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, rounds_str) = s.split_once(": ").ok_or(ParseError {
            message: format!("Failed to split header from rounds in {}", s),
        })?;
        let id = header
            .replace("Game ", "")
            .parse::<_>()
            .map_err(|_| ParseError {
                message: format!("Failed to parse game id in {}", s),
            })?;
        let mut rounds = vec![];
        for round_str in rounds_str.split("; ") {
            let mut reveal = CubeSet::default();
            for part in round_str.split(", ") {
                let next_reveal = CubeSet::from_str(part)?;
                reveal.add(&next_reveal);
            }
            rounds.push(reveal);
        }
        Ok(Game {
            id: id,
            rounds: rounds,
        })
    }
}

fn main() {
    let input = common::read_file("day-02/input.txt");
    let games = input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let real_amounts = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let part_1_answer = games
        .iter()
        .filter(|game| game.is_possible(&real_amounts))
        .map(|game| game.id)
        .sum::<u32>();
    println!("{}", part_1_answer);
    let part_2_answer = games
        .iter()
        .map(|game| game.get_minimum_set())
        .map(|set| set.power())
        .sum::<u32>();
    println!("{}", part_2_answer);
}

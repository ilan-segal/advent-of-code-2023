use common;
use std::{collections::HashSet, str::FromStr};

fn parse_numbers<'a>(space_separated_list: &'a str) -> impl Iterator<Item = u8> + 'a {
    space_separated_list
        .split_whitespace()
        .map(|n| n.parse::<u8>().unwrap())
}

struct ScratchCard {
    winning_numbers: HashSet<u8>,
    numbers: Vec<u8>,
}

impl ScratchCard {
    fn get_winning_number_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn get_points(&self) -> u32 {
        let num_matches = self.get_winning_number_count();
        if num_matches == 0 {
            0
        } else {
            1 << (num_matches - 1)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {
    message: String,
}

impl FromStr for ScratchCard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, tail) = s.split_once(": ").ok_or(ParseError {
            message: format!("Failed to split header from {}", s),
        })?;
        let (winning_numbers_str, numbers_str) = tail
            .split_once(" | ")
            .ok_or(ParseError {
                message: format!("Failed to split winning numbers off of {}", tail),
            })?;
        Ok(ScratchCard {
            winning_numbers: parse_numbers(winning_numbers_str).collect::<_>(),
            numbers: parse_numbers(numbers_str).collect::<_>(),
        })
    }
}

fn main() {
    let input = common::read_file("day-04/input.txt");
    let scratch_cards = input
        .lines()
        .map(|line| ScratchCard::from_str(line).unwrap())
        .collect::<Vec<_>>();

    let part_1_answer = scratch_cards
        .iter()
        .map(|c| c.get_points())
        .sum::<u32>();
    println!("{}", part_1_answer);

    let mut card_counts: Vec<u32> = vec![1; scratch_cards.len()];
    for (i, card) in scratch_cards.iter().enumerate() {
        let amount_of_this_card = card_counts[i];
        let win_count = card.get_winning_number_count();
        for j in 1..=win_count {
            card_counts[i + j] += amount_of_this_card;
        }
    }
    let part_2_answer = card_counts.iter().sum::<u32>();
    println!("{}", part_2_answer);
}

use std::collections::HashSet;

use common;

type Coordinates = (usize, usize);

fn get_neighbors(row: usize, col: usize) -> Vec<Coordinates> {
    let mut neighbors = vec![];
    for row_off in -1..=1 {
        if row == 0 && row_off < 0 {
            continue;
        }
        for col_off in -1..=1 {
            if col == 0 && col_off < 0 {
                continue;
            }
            if row_off == 0 && col_off == 0 {
                continue;
            }
            neighbors.push((
                (row as i16 + row_off) as usize,
                (col as i16 + col_off) as usize,
            ))
        }
    }
    return neighbors;
}

fn get_eligible_number_locations(schematics: &str) -> HashSet<Coordinates> {
    schematics
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(move |(j, char)| ((i, j), char))
        })
        .filter(|(_, char)| !char.is_numeric() && char != &'.')
        .flat_map(|((row, col), _)| get_neighbors(row, col))
        .collect::<_>()
}

struct SchematicNumber {
    number: u32,
    positions: Vec<Coordinates>,
}

impl SchematicNumber {
    fn is_valid(&self, eligible_positions: &HashSet<Coordinates>) -> bool {
        self.positions
            .iter()
            .any(|pos| eligible_positions.contains(pos))
    }

    fn is_neighboring(&self, pos: &Coordinates) -> bool {
        self.positions
            .iter()
            .flat_map(|(row, col)| get_neighbors(*row, *col))
            .collect::<HashSet<_>>()
            .contains(pos)
    }
}

fn get_schematic_numbers(input: &str) -> Vec<SchematicNumber> {
    let mut numbers = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut cur_number = 0u32;
        let mut cur_positions = vec![];
        for (col, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                cur_number = 10 * cur_number + digit;
                cur_positions.push((row, col));
            } else if cur_number != 0 {
                // Add new schematic number to return vector
                numbers.push(SchematicNumber {
                    number: cur_number,
                    positions: cur_positions.clone(),
                });
                cur_number = 0;
                cur_positions.clear();
            }
        }
        if cur_number != 0 {
            // Add new schematic number to return vector
            numbers.push(SchematicNumber {
                number: cur_number,
                positions: cur_positions.clone(),
            });
        }
    }
    return numbers;
}

fn sum_gear_ratios(input: &str, schematic_numbers: &Vec<SchematicNumber>) -> u32 {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| char == &'*')
                .map(move |(col, _)| (row, col))
        })
        .map(|pos| {
            schematic_numbers
                .iter()
                .filter(|num| num.is_neighboring(&pos))
                .collect::<Vec<_>>()
        })
        .filter_map(|neighbors| {
            if neighbors.len() == 2 {
                Some(
                    neighbors
                        .iter()
                        .map(|n| n.number)
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = common::read_file("day-03/input.txt");
    let eligible_positions = get_eligible_number_locations(&input);
    let schematic_numbers = get_schematic_numbers(&input);
    let part_1_answer: u32 = schematic_numbers
        .iter()
        .filter(|num| num.is_valid(&eligible_positions))
        .map(|num| num.number)
        .sum();
    println!("{}", part_1_answer);
    let part_2_answer = sum_gear_ratios(&input, &schematic_numbers);
    println!("{}", part_2_answer);
}

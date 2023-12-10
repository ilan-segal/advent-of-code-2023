use std::{collections::HashMap, str::FromStr};

use common;

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, pair) = s.split_once(" = ").ok_or(ParseError)?;
        let binding = pair.replace("(", "").replace(")", "");
        let (left, right) = binding
            .split_once(", ")
            .ok_or(ParseError)?;
        Ok(Node {
            id: String::from(id),
            left: String::from(left),
            right: String::from(right),
        })
    }
}

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Network {
    nodes_by_id: HashMap<String, Node>,
}

impl From<&[Node]> for Network {
    fn from(nodes: &[Node]) -> Self {
        Network {
            nodes_by_id: nodes
                .iter()
                .map(|node| (node.id.clone(), node.clone()))
                .collect(),
        }
    }
}

impl Network {
    fn go_left_from(&self, node_id: &str) -> Option<&String> {
        self.nodes_by_id
            .get(node_id)
            .map(|node| &node.left)
    }

    fn go_right_from(&self, node_id: &str) -> Option<&String> {
        self.nodes_by_id
            .get(node_id)
            .map(|node| &node.right)
    }

    fn measure_walk(&self, instructions: &[Instruction]) -> Option<usize> {
        let repeating_instructions = instructions.iter().cycle();
        let mut num_steps = 0;
        let mut cur_node_id = &String::from("AAA");
        for instruction in repeating_instructions {
            if cur_node_id == "ZZZ" {
                return Some(num_steps);
            }
            match instruction {
                Instruction::Left => cur_node_id = self.go_left_from(&cur_node_id).unwrap(),
                Instruction::Right => {
                    cur_node_id = self
                        .go_right_from(&cur_node_id)
                        .unwrap()
                }
            }
            num_steps += 1;
        }
        // In case there are no elements to loop over
        return None;
    }

    fn measure_ghost_walk(&self, instructions: &[Instruction]) -> Option<usize> {
        let repeating_instructions = instructions.iter().cycle();
        let mut num_steps = 0;
        let mut cur_node_id = self
            .nodes_by_id
            .values()
            .map(|node| node.id.clone())
            .filter(|id| id.ends_with("A"))
            .collect::<Vec<_>>();
        for instruction in repeating_instructions {
            if cur_node_id
                .iter()
                .all(|id| id.ends_with("Z"))
            {
                return Some(num_steps);
            }
            cur_node_id = cur_node_id
                .iter()
                .map(|id| match instruction {
                    Instruction::Left => self.go_left_from(&id).unwrap(),
                    Instruction::Right => self.go_right_from(&id).unwrap(),
                })
                .cloned()
                .collect::<_>();
            num_steps += 1;
        }
        // In case there are no elements to loop over
        return None;
    }
}

fn parse_input(input: &str) -> Option<(Vec<Instruction>, Network)> {
    let mut lines = input.lines();
    let instruction_str = lines.next()?;
    let instructions = instruction_str
        .chars()
        .map(|c| Instruction::from(c))
        .collect::<_>();
    lines.next(); // Skip blank line
    let nodes = lines
        .map(|line| Node::from_str(line).unwrap())
        .collect::<Vec<_>>();

    Some((instructions, Network::from(nodes.as_slice())))
}

fn main() {
    let input = common::read_file("day-08/input.txt");
    let (instructions, network) = parse_input(&input).unwrap();
    let part_1_answer = network
        .measure_walk(&instructions.as_slice())
        .unwrap();
    println!("{}", part_1_answer);
    // let part_2_answer = network
    //     .measure_ghost_walk(&instructions.as_slice())
    //     .unwrap();
    // println!("{}", part_2_answer);
}

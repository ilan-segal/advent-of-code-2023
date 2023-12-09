use std::str::{FromStr, Lines};

use common;

#[derive(Clone, Copy, Debug)]
struct MappingRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

#[derive(PartialEq, Eq)]
enum RangeQueryResult {
    LeftOf,
    RightOf,
    Contains,
}

impl MappingRange {
    fn query(&self, point: u64) -> RangeQueryResult {
        if point < self.source_start {
            RangeQueryResult::RightOf
        } else if self.source_start + self.length < point {
            RangeQueryResult::LeftOf
        } else {
            RangeQueryResult::Contains
        }
    }

    fn map(&self, point: u64) -> u64 {
        match self.query(point) {
            RangeQueryResult::Contains => self.destination_start + point - self.source_start,
            _ => panic!("Query {} falls outside of range {:?}", point, self),
        }
    }

    fn fully_contains(&self, start: &u64, end: &u64) -> bool {
        self.source_start <= *start && (self.source_start + self.length) >= *end
    }

    fn map_range(&self, other: &MappingRange) -> Vec<MappingRange> {
        // Vector of all points which are on (or adjacent to) a boundary of either range.
        // The boundaries of the split ranges are guaranteed to be in this vector.
        let mut critical_points = vec![
            self.source_start,
            // Last point in self
            self.source_start + self.length - 1,
            // First point after end of self
            self.source_start + self.length,
            // First point in other
            other.source_start,
            // Last point in other
            other.source_start + other.length - 1,
            // First point after end of other
            other.source_start + other.length,
        ];
        if self.source_start > 0 {
            // Last point before start of self
            critical_points.push(self.source_start - 1);
        }
        if other.source_start > 0 {
            // Last point before start of other
            critical_points.push(other.source_start - 1);
        }
        critical_points.sort();
        critical_points.dedup();
        critical_points
            .windows(2)
            .map(|slice| (slice[0], slice[1]))
            .filter(|(start, end)| other.fully_contains(start, end))
            .map(|(start, end)| {
                if self.fully_contains(&start, &end) {
                    MappingRange {
                        source_start: start,
                        destination_start: self.map(start),
                        length: end - start,
                    }
                } else {
                    MappingRange {
                        source_start: start,
                        destination_start: start,
                        length: end - start,
                    }
                }
            })
            .collect::<_>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {
    message: String,
}

impl FromStr for MappingRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split_whitespace()
            .map(|num| num.parse::<u64>());
        let destination_start = nums
            .next()
            .expect("Expected expected number")
            .map_err(|_| ParseError {
                message: format!("Failed to parse first number from {}", s),
            })?;
        let source_start = nums
            .next()
            .expect("Expected first number")
            .map_err(|_| ParseError {
                message: format!("Failed to parse second number from {}", s),
            })?;
        let length = nums
            .next()
            .expect("Expected third number")
            .map_err(|_| ParseError {
                message: format!("Failed to parse third number from {}", s),
            })?;
        if nums.next() != None {
            Result::Err(ParseError {
                message: format!("Unexpected fields after first 3 numbers in {}", s),
            })
        } else {
            Result::Ok(MappingRange {
                source_start: source_start,
                destination_start: destination_start,
                length: length,
            })
        }
    }
}

#[derive(Debug)]
struct RangeTreeNode {
    center: u64,
    left: Option<Box<RangeTreeNode>>,
    right: Option<Box<RangeTreeNode>>,
    overlapping_sorted_start: Vec<MappingRange>,
    overlapping_sorted_end: Vec<MappingRange>,
}

impl RangeTreeNode {
    fn create_tree(ranges: Vec<MappingRange>) -> Option<RangeTreeNode> {
        // https://en.wikipedia.org/wiki/Interval_tree#Construction
        if ranges.is_empty() {
            return None;
        }
        let min = ranges
            .iter()
            .map(|range| range.source_start)
            .min()?;
        let max = ranges
            .iter()
            .map(|range| range.source_start)
            .max()?;
        let center = (min + max) >> 1;
        let mut left = vec![];
        let mut right = vec![];
        let mut overlapping = vec![];
        for range in ranges {
            match range.query(center) {
                RangeQueryResult::LeftOf => left.push(range),
                RangeQueryResult::RightOf => right.push(range),
                RangeQueryResult::Contains => overlapping.push(range),
            }
        }
        let left_tree = RangeTreeNode::create_tree(left);
        let right_tree = RangeTreeNode::create_tree(right);
        let mut overlapping_sorted_start = overlapping.clone();
        overlapping_sorted_start.sort_by_key(|r| r.source_start);
        let mut overlapping_sorted_end = overlapping.clone();
        overlapping_sorted_end.sort_by_key(|r| -(r.source_start as i128));
        Some(RangeTreeNode {
            center: center,
            left: left_tree.map(|v| Box::new(v)),
            right: right_tree.map(|v| Box::new(v)),
            overlapping_sorted_start: overlapping_sorted_start,
            overlapping_sorted_end: overlapping_sorted_end,
        })
    }

    fn map(&self, x: u64) -> Option<u64> {
        if x < self.center {
            self.overlapping_sorted_start
                .iter()
                .take_while(|range| range.query(x) == RangeQueryResult::Contains)
                .map(|range| range.map(x))
                .next()
                .or_else(|| {
                    self.left
                        .as_ref()
                        .and_then(|node| node.map(x))
                })
        } else if x > self.center {
            self.overlapping_sorted_end
                .iter()
                .take_while(|range| range.query(x) == RangeQueryResult::Contains)
                .map(|range| range.map(x))
                .next()
                .or_else(|| {
                    self.right
                        .as_ref()
                        .and_then(|node| node.map(x))
                })
        } else {
            self.overlapping_sorted_start
                .iter()
                .take_while(|range| range.query(x) == RangeQueryResult::Contains)
                .map(|range| range.map(x))
                .next()
        }
    }

    fn get_overlapping_ranges(&self, range: &MappingRange) -> Vec<MappingRange> {
        // https://en.wikipedia.org/wiki/Interval_tree#With_an_interval
        todo!()
    }
}

fn get_range_tree(lines: &Lines<'_>, header: &str) -> RangeTreeNode {
    let ranges = lines
        .clone()
        .skip_while(|line| *line != header)
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| MappingRange::from_str(line).unwrap())
        .collect::<_>();
    RangeTreeNode::create_tree(ranges).unwrap()
}

fn feed_forward(trees: &Vec<RangeTreeNode>, x: u64) -> u64 {
    let mut cur = x;
    for tree in trees {
        cur = tree.map(cur).unwrap_or(cur);
    }
    return cur;
}

fn main() {
    let input = common::read_file("day-05/input.txt");
    let lines = input.lines();

    let seed_numbers = lines
        .clone()
        .next()
        .expect("First line: seed numbers")
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = vec![
        get_range_tree(&lines, "seed-to-soil map:"),
        get_range_tree(&lines, "soil-to-fertilizer map:"),
        get_range_tree(&lines, "fertilizer-to-water map:"),
        get_range_tree(&lines, "water-to-light map:"),
        get_range_tree(&lines, "light-to-temperature map:"),
        get_range_tree(&lines, "temperature-to-humidity map:"),
        get_range_tree(&lines, "humidity-to-location map:"),
    ];

    let part_1_answer = seed_numbers
        .iter()
        // .take(1)
        .map(|seed| feed_forward(&maps, *seed))
        .min()
        .expect("Minimum location");

    println!("{}", part_1_answer);
}

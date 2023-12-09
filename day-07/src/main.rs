use core::{fmt, panic};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait GetHandType
where
    Self: Sized,
{
    fn get_hand_type(cards: &[Self]) -> HandType;
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {
    message: String,
}

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Copy, Debug)]
enum Part1Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Part1Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Part1Card::Ace),
            "K" => Ok(Part1Card::King),
            "Q" => Ok(Part1Card::Queen),
            "J" => Ok(Part1Card::Jack),
            "T" => Ok(Part1Card::Number(10)),
            _ => {
                let parse = s.parse::<u8>();
                let number = parse.map_err(|_| ParseError {
                    message: format!("Cannot get card number from {}", s),
                })?;
                if number < 2 || number > 10 {
                    Err(ParseError {
                        message: format!("Invalid card number {}", s),
                    })
                } else {
                    Ok(Part1Card::Number(number))
                }
            }
        }
    }
}

impl GetHandType for Part1Card {
    fn get_hand_type(cards: &[Self]) -> HandType {
        let mut count_per_card: HashMap<Part1Card, u8> = HashMap::default();
        for card in cards {
            count_per_card
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut counts: Vec<_> = count_per_card.values().collect::<_>();
        counts.sort();
        counts.reverse();
        match counts[..] {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [] => panic!("Cannot determine hand type from empty hand"),
            _ => HandType::HighCard,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Copy, Debug)]
enum Part2Card {
    Joker,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl FromStr for Part2Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Part2Card::Ace),
            "K" => Ok(Part2Card::King),
            "Q" => Ok(Part2Card::Queen),
            "J" => Ok(Part2Card::Joker),
            "T" => Ok(Part2Card::Number(10)),
            _ => {
                let parse = s.parse::<u8>();
                let number = parse.map_err(|_| ParseError {
                    message: format!("Cannot get card number from {}", s),
                })?;
                if number < 2 || number > 10 {
                    Err(ParseError {
                        message: format!("Invalid card number {}", s),
                    })
                } else {
                    Ok(Part2Card::Number(number))
                }
            }
        }
    }
}

impl Part2Card {
    fn joker_replacements() -> impl Iterator<Item = Part2Card> {
        [
            Part2Card::Number(2),
            Part2Card::Number(3),
            Part2Card::Number(4),
            Part2Card::Number(5),
            Part2Card::Number(6),
            Part2Card::Number(7),
            Part2Card::Number(8),
            Part2Card::Number(9),
            Part2Card::Number(10),
            Part2Card::Queen,
            Part2Card::King,
            Part2Card::Ace,
        ]
        .into_iter()
    }

    fn get_possible_hands(cards: &[Self]) -> Vec<Vec<Self>> {
        let index_of_joker = cards
            .iter()
            .position(|c| *c == Part2Card::Joker);
        if let Some(index) = index_of_joker {
            let possible_replacements = Part2Card::joker_replacements();
            let mut results = vec![];
            for card in possible_replacements {
                let new_cards = [&cards[..index], &[card], &cards[index + 1..]].concat();
                let sub_results = Self::get_possible_hands(&new_cards);
                results.extend(sub_results)
            }
            return results;
        } else {
            vec![cards.to_vec()]
        }
    }
}

impl GetHandType for Part2Card {
    fn get_hand_type(cards: &[Self]) -> HandType {
        // println!("{:?}", cards);
        let possibilities = Part2Card::get_possible_hands(cards);
        // println!("{:?}", possibilities);
        possibilities
            .iter()
            .map(|possible_cards| {
                let mut count_per_card: HashMap<Self, u8> = HashMap::default();
                for card in possible_cards {
                    count_per_card
                        .entry(*card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                let mut counts: Vec<_> = count_per_card.values().collect::<_>();
                counts.sort();
                counts.reverse();
                match counts[..] {
                    [5, ..] => HandType::FiveOfAKind,
                    [4, ..] => HandType::FourOfAKind,
                    [3, 2, ..] => HandType::FullHouse,
                    [3, ..] => HandType::ThreeOfAKind,
                    [2, 2, ..] => HandType::TwoPair,
                    [2, ..] => HandType::OnePair,
                    [] => panic!("Cannot determine hand type from empty hand"),
                    _ => HandType::HighCard,
                }
            })
            .sorted()
            .last()
            .unwrap()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand<const SIZE: usize, CardType> {
    hand_type: HandType,
    cards: [CardType; SIZE],
    bid: u16,
}

impl<const SIZE: usize, CardType: PartialOrd> PartialOrd for Hand<SIZE, CardType> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_type
            .partial_cmp(&other.hand_type)
            .filter(|ord| ord.is_ne())
            .or_else(|| {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .filter_map(|(a, b)| a.partial_cmp(b))
                    .filter(|ord| ord.is_ne())
                    .next()
            })
    }
}

impl<const SIZE: usize, CardType: PartialOrd + Eq> Ord for Hand<SIZE, CardType> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl<const SIZE: usize, CardType: FromStr + std::hash::Hash + GetHandType> FromStr
    for Hand<SIZE, CardType>
where
    <CardType as FromStr>::Err: fmt::Debug,
{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s
            .split_once(" ")
            .ok_or_else(|| ParseError {
                message: format!("Cannot get hand from {}", s),
            })?;
        let cards: [CardType; SIZE] =
            std::array::from_fn(|i| CardType::from_str(&cards_str[i..i + 1]).unwrap());
        let hand_type = CardType::get_hand_type(cards.as_slice());
        let bid = bid_str
            .parse::<u16>()
            .map_err(|_| ParseError {
                message: format!("Cannot get hand from {}", s),
            })?;
        Ok(Hand {
            hand_type: hand_type,
            cards: cards,
            bid: bid,
        })
    }
}

fn get_total_bid<const SIZE: usize, CardType: PartialOrd + Eq + FromStr>(
    hands: &Vec<Hand<SIZE, CardType>>,
) -> u128 {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, e)| (i + 1, e))
        .map(|(rank, hand)| rank as u128 * hand.bid as u128)
        .sum::<_>()
}

fn main() {
    let input = common::read_file("day-07/input.txt");
    let hands_1: Vec<Hand<5, Part1Card>> = input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect::<_>();

    let part_1_answer = get_total_bid(&hands_1);
    println!("{}", part_1_answer);

    let hands_2: Vec<Hand<5, Part2Card>> = input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect::<_>();

    let part_2_answer = get_total_bid(&hands_2);
    println!("{}", part_2_answer);
}

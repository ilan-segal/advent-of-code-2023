use common;

fn get_number_from_line(line: &str) -> Option<u32> {
    let mut digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .peekable();
    let first_digit = *digits.peek()?;
    let last_digit = digits.last()?;
    Some(10 * first_digit + last_digit)
}

fn replace_words_with_digits(word: &str) -> String {
    if word.len() < 3 {
        return String::from(word);
    }
    let replacement = {
        if word.starts_with("one") {
            Some("1")
        } else if word.starts_with("two") {
            Some("2")
        } else if word.starts_with("three") {
            Some("3")
        } else if word.starts_with("four") {
            Some("4")
        } else if word.starts_with("five") {
            Some("5")
        } else if word.starts_with("six") {
            Some("6")
        } else if word.starts_with("seven") {
            Some("7")
        } else if word.starts_with("eight") {
            Some("8")
        } else if word.starts_with("nine") {
            Some("9")
        } else {
            None
        }
    };
    match replacement {
        Some(replacement) => replacement.to_owned() + &replace_words_with_digits(&word[1..]),
        None => word[0..1].to_owned() + &replace_words_with_digits(&word[1..]),
    }
}

fn get_number_from_line_2(line: &str) -> Option<u32> {
    let replaced = replace_words_with_digits(line);
    // println!("{replaced}");
    let mut digits = replaced
        .chars()
        .filter_map(|c| c.to_digit(10))
        .peekable();
    let first_digit = *digits.peek()?;
    let last_digit = digits.last()?;
    Some(10 * first_digit + last_digit)
}

fn main() {
    let input = common::read_file("day-01/input.txt");
    let sum = input
        .trim()
        .lines()
        .filter_map(|line| get_number_from_line(line))
        .sum::<u32>();
    println!("{sum}");
    let sum_2 = input
        .trim()
        .lines()
        .filter_map(|line| get_number_from_line_2(line))
        .sum::<u32>();
    println!("{sum_2}");
}

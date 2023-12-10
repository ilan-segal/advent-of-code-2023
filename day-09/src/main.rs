use common;

fn extrapolate(series: &[i32]) -> (i32, i32) {
    if series.is_empty() {
        panic!("Cannot extrapolate from empty series");
    }
    let first = &series[0];
    let is_constant = series.iter().all(|item| item == first);
    if is_constant {
        return (series[0], series[0]);
    }
    let differences = series
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();
    let next_differences = extrapolate(differences.as_slice());
    let last = series
        .last()
        .expect("Series should not be empty");
    return (first - next_differences.0, last + next_differences.1);
}

fn main() {
    let input = common::read_file("day-09/input.txt");
    let series_arrays = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| {
                    v.parse::<i32>()
                        .expect("Input file should contain valid integers")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answers = series_arrays
        .iter()
        .map(|series| extrapolate(series.as_slice()))
        .reduce(|(acc_first, acc_last), (first, last)| (acc_first + first, acc_last + last))
        .expect("Given series should all be extractable");

    println!("{}", answers.1);
    println!("{}", answers.0);
}

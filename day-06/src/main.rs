use common;

#[derive(Debug)]
struct Race {
    time_limit: u64,
    record_distance: u64,
}

impl Race {
    /**
     * We can define distance travelled for any button_hold_time as:
     *
     *  distance = (time_limit - button_hold_time) * button_hold_time
     *
     * We can also define the inverse mapping, restricting button_hold_time
     * to positive values:
     *
     *  button_hold_time = 0.5 * (time_limit ± sqrt(time_limit^2 - 4 * distance))
     */
    fn get_record_button_hold_time(&self) -> (f64, f64) {
        let a = 0.5 * self.time_limit as f64;
        let b =
            ((self.time_limit * self.time_limit - 4 * self.record_distance) as f64).sqrt() * 0.5;
        (a - b, a + b)
    }

    fn count_ways_to_beat_record(&self) -> u64 {
        let (a, b) = self.get_record_button_hold_time();
        let minimum_time = (a + 1.0).floor() as u64;
        let maximum_time = (b - 1.0).ceil() as u64;
        let result = maximum_time - minimum_time + 1;
        // println!(
        //     "{:?} record-holding button time one of: {}, {}. {} ways to beat.",
        //     self, a, b, result
        // );
        return result;
    }
}

fn get_races(s: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| Race {
            time_limit: time,
            record_distance: distance,
        })
}

fn get_race_part_2(s: &str) -> Race {
    let mut lines = s.lines();
    let time = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<_>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<_>()
        .unwrap();
    Race {
        time_limit: time,
        record_distance: distance,
    }
}

fn main() {
    let input = common::read_file("day-06/input.txt");

    let races = get_races(&input).collect::<Vec<_>>();
    let part_1_answer = races
        .iter()
        .map(|r| r.count_ways_to_beat_record() as u64)
        .product::<u64>();
    println!("{}", part_1_answer);

    let race = get_race_part_2(&input);
    let part_2_answer = race.count_ways_to_beat_record();
    println!("{}", part_2_answer);
}

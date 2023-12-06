use input;
use regex::Regex;

fn main() {
    let input = input::get_input_lines();
    let mut lines = input.lines();

    let timing_line = lines.next().unwrap();
    let record_line = lines.next().unwrap();

    let re = Regex::new(r"(\d+)").unwrap();

    let mut times: Vec<usize> = vec![];
    let mut records: Vec<usize> = vec![];
    for (x, _) in re.captures_iter(timing_line.split(": ").last().unwrap()).map(|c| c.extract::<1>()) {
        times.push(x.parse().unwrap());
    }
    for (x, _) in re.captures_iter(record_line.split(": ").last().unwrap()).map(|c| c.extract::<1>()) {
        records.push(x.parse().unwrap());
    }

    let games: Vec<(&usize, &usize)> = times.iter().zip(records.iter()).collect();

    // part 1
    let mut total = 1;
    for (time, record) in games {
        // brute force it
        let mut ways_to_win = 0;
        for ms in 1..*time {
            let time_left = time - ms;
            let distance = ms * time_left;

            if distance > *record {
                ways_to_win += 1;
            }
        }
        total *= ways_to_win;
    }
    dbg!(total);
}

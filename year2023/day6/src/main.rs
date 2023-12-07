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

    // part 2
    // 1. find min speed + seconds needed
    // 2. find max speed that still leaves enough time
    //
    // approaches
    //   - approach from both sides, start with big steps and make them smaller until exact match?
    //   - min = round_up(distance/time)?
    //   - max = time - min?
    //
    // Wrong answers
    // - 32001936 - too high

    fn reduce_string(a: String, b: String) -> String {
        let mut o = a.to_owned();
        o.push_str(b.as_str());
        return o;
    }
    let big_game_time: i64 = times.iter().map(|c| c.to_string() ).reduce(reduce_string).unwrap().parse().unwrap();
    let big_game_record: i64 = records.iter().map(|c| c.to_string() ).reduce(reduce_string).unwrap().parse().unwrap();

    dbg!(big_game_time, big_game_record);

    // round up
    let min = big_game_record / big_game_time + (big_game_record % big_game_time).signum();
    let max = big_game_time - min;

    dbg!(min, max, max-min+1);
}

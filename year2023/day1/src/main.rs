use std::{fs, process::exit, env};

fn main() {
    let file: &str = match env::var("USE_EXAMPLE") {
        Ok(s) => match s.as_str() {
            "part2" => "src/example2.txt",
            _ => "src/example.txt",
        },
        Err(_) => "src/input.txt",
    };

    println!("file: {}", file);
    let input = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => {
            println!("unable to find file {}", file);
            exit(1);
        }
    };

    let needles = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total: u32 = 0;
    for line in input.lines().into_iter() {

        let mut first_matches: Vec<(usize, &str)> = vec![];
        let mut last_matches: Vec<(usize, &str)> = vec![];
        for nr in needles.iter() {
            let needle_matches = line.match_indices(nr).collect::<Vec<_>>();
            if needle_matches.len() > 0 {
                first_matches.push(needle_matches[0]);
            }
            let needle_matches = line.rmatch_indices(nr).collect::<Vec<_>>();
            if needle_matches.len() > 0 {
                last_matches.push(needle_matches[0]);
            }
        }
        first_matches.sort_by_key(|m|m.0);
        last_matches.sort_by(|a,b| b.0.cmp(&a.0));

        let first_nr = match needles.iter().position(|&x|x==first_matches[0].1) {
            Some(index) => needles[if index >=9 { index - 9 } else { index }],
            None => "??"
        };
        let last_nr = match needles.iter().position(|&x|x==last_matches[0].1) {
            Some(index) => needles[if index >=9 { index - 9 } else { index }],
            None => "??"
        };
        let parsed_number = [first_nr, last_nr].concat().parse::<u32>().unwrap();
        println!("Line: {} => {} {} = {}", line, first_matches[0].1, last_matches[0].1, parsed_number);
        total += parsed_number
    }
    println!("Got number: {}", total);
}

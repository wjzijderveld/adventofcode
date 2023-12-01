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

    // let number_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // let inversed_number_words = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
    // let number_strings = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
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
        /*
        let mut number: String = "".to_owned();
        for (i, nr) in [number_words.clone(), number_strings.clone()].concat().iter().enumerate() {
            let index = match line.find(nr) {
                Some(_) => i,
                None => continue,
            };
            number.push_str(number_strings[if index >= 9 { index - 9 } else { index }]);
            println!("First: {}", index);
            break;
        }
        for (i, nr) in [inversed_number_words.clone(), number_strings.clone()].concat().iter().enumerate() {
            let reversed_line = line.chars().rev().collect::<String>();
            println!("finding {} in {}", nr, reversed_line);
            let index = match reversed_line.find(nr) {
                Some(_) => i,
                None => continue,
            };
            println!("Index: {}", index);
            number.push_str(number_strings[if index >= 9 { index - 9 } else { index }]);
            println!("Last: {}", number_strings[if index >= 9 { index - 9 } else { index }]);
            break;
        }
        println!("Number: {}", number);
        total = total + number.parse::<u32>().unwrap();
        */
    }
    println!("Got number: {}", total);
}

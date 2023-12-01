use std::{fs, process::exit, env};

fn main() {
    let file: &str = match env::var("USE_EXAMPLE") {
        Ok(_) => "src/example.txt",
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

    let mut total: u32 = 0;
    for line in input.lines().into_iter() {
        let mut number: String = "".to_owned();
        for c in line.chars() {
            if c.is_digit(10) {
                number.push(c);
                break
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                number.push(c);
                break
            }
        }
        total = total + number.parse::<u32>().unwrap();
    }
    println!("Got number: {}", total);
}

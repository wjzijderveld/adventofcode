use std::{env, process, fs};

pub fn get_input_lines() -> String {
    let file: &str = match env::var("USE_EXAMPLE") {
        Ok(s) => match s.as_str() {
            "part2" => "src/example2.txt",
            _ => "src/example.txt",
        },
        Err(_) => "src/input.txt",
    };

let path = format!("{}", file);
    let input = match fs::read_to_string(path.clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("unable to find file {}", path);
            process::exit(1);
        }
    };
    
    return input;
}

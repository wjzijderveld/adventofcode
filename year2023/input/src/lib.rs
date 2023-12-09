use std::{env, process, fs};

pub fn get_input_lines() -> String {
    let file: String = match env::var("USE_EXAMPLE") {
        Ok(s) if file_exists(path(&s).as_str()) => path(&s),
        Ok(_) => String::from("src/example.txt"),
        Err(_) => String::from("src/input.txt"),
    };

    println!("Using {}", file);
    let input = match fs::read_to_string(file.clone()) {
        Ok(s) => s,
        Err(_) => {
            println!("unable to find file {}", file);
            process::exit(1);
        }
    };
    
    return input;
}

fn path(file: &str) -> String {
    "src/".to_owned() + file + ".txt"
}

fn file_exists(file: &str) -> bool {
    fs::metadata(file).is_ok()
}

pub fn lap(name: String, started_at: std::time::Instant) {
    println!("{} => {}ms", name, started_at.elapsed().as_millis());
}

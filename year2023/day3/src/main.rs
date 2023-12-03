use input;

#[derive(Copy, Clone, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Number {
    number: Vec<char>,
    started_at: Loc,
}

impl Number {
    fn new(started_at: Loc, first_char: char) -> Self {
        Self {
            number: vec![first_char],
            started_at,
        }
    }

    fn add_char(&mut self, add: char) {
        self.number.push(add);
    }
}

fn main() {
    let i = input::get_input_lines();
    println!("Input: {}", i);

    let mut schematic: Vec<Vec<char>> = vec![];
    let mut numbers_found_at: Vec<Loc> = vec![];
    // let mut symbols_found_at: Vec<Loc> = vec![];
    for (i, line) in i.lines().enumerate() {
        schematic.push(vec![]);
        let mut peekable_chars = line.chars().peekable();
        let mut j = 0;
        let mut current_number: Option<Number> = None;
        loop {
            match peekable_chars.next() {
                Some(c) => {
                    // keep track of individual characters
                    schematic[i].push(c);

                    // collect all digits of the number
                    if c.is_numeric() {
                        match current_number {
                            Some(ref mut cn) => cn.add_char(c),
                            None => {
                                let loc = Loc::new(i, j);
                                let num = Number::new(loc, c);
                                current_number = Some(num);
                                numbers_found_at.push(loc)
                            }
                        }
                    }

                    j += 1;
                    // peek next char to process whole number
                    // peeking here to avoid dealing with EOL/EOF fun outside the loops
                    let peek = peekable_chars.peek();
                    match peek {
                        Some(x) if x.is_numeric() => continue, // number is not finished, continue
                                                               // to next iteration
                        Some(_) | None => current_number = None,   // dot or symbol
                    }
                },
                None => break, // no next character, break loop
            }
        }
    }

    println!("Numbers found: {:?}", numbers_found_at);
}

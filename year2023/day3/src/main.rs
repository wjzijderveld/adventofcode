use std::collections::HashMap;

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

    fn as_index(&self) -> String {
        format!("x{}y{}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
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

    fn get_number(&self) -> usize {
        self.number
            .iter()
            .cloned()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn get_search_space(&self, schematic: Vec<Vec<char>>) -> (Loc, Loc) {
        let mut start_y = 0;
        let mut start_x = 0;
        if self.started_at.y > 0 {
            // not in top row
            start_y = self.started_at.y - 1;
        }
        if self.started_at.x > 0 {
            start_x = self.started_at.x - 1;
        }

        let mut end_x = self.started_at.x + self.number.len();
        let mut end_y = self.started_at.y + 1;
        if end_x >= schematic[0].len() - 1 {
            end_x = schematic[0].len() - 1;
        }
        if end_y >= schematic.len() - 1 {
            end_y = schematic.len() - 1;
        }

        (Loc::new(start_x, start_y), Loc::new(end_x, end_y))
    }

    fn is_adjacent_to_symbol(&self, schematic: Vec<Vec<char>>) -> Option<(char, Loc)> {
        let (start, end) = self.get_search_space(schematic.clone());

        for x in start.x..end.x + 1 {
            for y in start.y..end.y + 1 {
                match schematic[y][x] {
                    '0'..='9' | '.' => {} // ignore numbers and dots
                    c => return Some((c, Loc::new(x, y))),
                }
            }
        }

        None
    }
}

// wrong answers:
// - 567240 => too high
// - 446820 => too low
//
// third and correct guess:
// - 556057
fn main() {
    let i = input::get_input_lines();

    // println!("{}", i);

    let mut schematic: Vec<Vec<char>> = vec![];
    let mut found_numbers: Vec<Number> = vec![];
    let mut found_gears: Vec<Loc> = vec![];
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

                    if c == '*' {
                        found_gears.push(Loc::new(j, i));
                    }

                    // collect all digits of the number
                    if ('0'..='9').contains(&c) {
                        match current_number {
                            Some(ref mut cn) => cn.add_char(c),
                            None => {
                                let loc = Loc::new(j, i);
                                let num = Number::new(loc, c);
                                current_number = Some(num);
                            }
                        }
                    }

                    j += 1;
                    // peek next char to process whole number
                    // peeking here to avoid dealing with EOL/EOF fun outside the loops
                    let peek = peekable_chars.peek();
                    match peek {
                        Some(x) if ('0'..='9').contains(x) => continue, // number is not finished, continue
                        // to next iteration
                        Some(_) | None => {
                            // dot or symbol, collect buffered number if there is one
                            match current_number {
                                Some(ref mut cn) => found_numbers.push(cn.clone()),
                                None => {}
                            }
                            // reset number and start looking for the next one
                            current_number = None;
                        }
                    };
                }
                None => break, // no next character, break loop
            }
        }
    }

    let mut numbers_near_gears: HashMap<String, Vec<Number>> = HashMap::new();
    let mut total = 0;
    for number in found_numbers.clone() {
        let adjacent_symbol = number.is_adjacent_to_symbol(schematic.clone());
        if adjacent_symbol.is_some() {
            total += number.get_number();
            match adjacent_symbol.unwrap() {
                ('*', l) => {
                    let index = l.as_index();
                    numbers_near_gears.entry(index).and_modify(|v| v.push(number.clone())).or_insert(vec![number]);
                }
                _ => {}
            }
        }
    }
    // Found 556057
    println!("Sum of numbers adjecent to symbol: {}", total);

    let mut gear_total = 0;
    for (_, gear) in found_gears.clone().iter().enumerate() {
        let index = gear.as_index();
        match numbers_near_gears.get(&index) {
            Some(v) if v.len() == 2 => gear_total += v.clone().iter().map(|n|n.get_number()).product::<usize>(),
            _ => {}
        }
    }
    // Found 82824352
    println!("Sum of products of gears: {}", gear_total);
}

use std::collections::{HashSet, hash_map::RandomState};

use input;

#[derive(Debug)]
struct Game {
    id: usize,
    guessed_numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}


impl Game {
    fn new(id: usize, guessed_numbers: Vec<usize>, winning_numbers: Vec<usize>) -> Self {
        Self { id, guessed_numbers, winning_numbers }
    }

    fn get_score(&self) -> usize {
        let guessed = self.guessed_numbers.clone();
        let winning = self.winning_numbers.clone();

        let b: HashSet<usize, RandomState> = HashSet::from_iter(winning);
        let count = HashSet::from_iter(guessed).intersection(&b).count();
        let score = if count > 1 { 2_usize.pow((count-1).try_into().unwrap()) } else { count };
        if std::env::var("DEBUG").is_ok() {
            println!("Score game {} => {}: {}", self.id, count, score);
        }

        score
    }
}

fn main() {
    let i = input::get_input_lines();

    let mut total = 0;
    for line in i.lines() {
        let game = match line.split(": ").collect::<Vec<&str>>().as_slice() {
            [game_description, cards] => {
                let (guessed, winning) = match cards.split(" | ").collect::<Vec<&str>>().as_slice() {
                    [winning_string, guessed_string] => {
                        (
                            winning_string.split(" ").filter(|c| *c != "").map(|x| x.parse().unwrap()).collect(),
                            guessed_string.split(" ").filter(|c| *c != "").map(|x| x.parse().unwrap()).collect(),
                        )
                    }
                    ,
                    _ => panic!("invalid game")
                };
                Game::new(
                    match game_description.split(" ").filter(|c| *c != "").collect::<Vec<&str>>().as_slice() {
                        [_, x] => x.parse().unwrap(),
                        _ => panic!("invalid game")
                    },
                    guessed,
                    winning,
                )
            },
            _ => panic!("invalid game")
        };
        total += game.get_score();
    }
    println!("Total: {}", total);
}

use std::collections::{HashSet, BTreeMap, HashMap, hash_map::RandomState};

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

    fn get_count(&self) -> usize {
        let guessed = self.guessed_numbers.clone();
        let winning = self.winning_numbers.clone();

        let b: HashSet<usize, RandomState> = HashSet::from_iter(winning);
        HashSet::from_iter(guessed).intersection(&b).count()
    }
}

// Wrong guess:
// - 2794230 - too low
//
// Correct guess:
// - 5625994
fn main() {
    let i = input::get_input_lines();

    let mut card_copies = HashMap::new();
    let mut games = BTreeMap::new();
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
        card_copies.insert(game.id, 1);
        games.insert(game.id, game);
    }

    for (id, game) in games {
        let count = game.get_count();
        // println!("game={} count={} copies={:?}", id, count, card_copies.clone());
        for next_id in id+1..=(id+count) {
            let immutable_card_copies = card_copies.clone();
            card_copies.entry(next_id).and_modify(|x: &mut i32| *x += immutable_card_copies.get(&id).unwrap_or(&0));
        }
    }
    let total_cards: i32 = card_copies.into_values().sum();

    println!("Total: {}", total_cards);
}

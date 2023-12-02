use input;
use std::fmt;

fn main() {
    let i = input::get_input_lines();
    println!("Input: {}", i);


    // first collect all the info
    let mut games: Vec<Game> = vec![];
    for line in i.lines() {
        let mut game_parts = line.split(": ");
        let game_number: usize = match game_parts.next() {
            Some(game) => game.split(' ').last().unwrap().parse().unwrap(),
            None => panic!("game doesn't have a number")
        };
        let rounds = game_parts.next().unwrap().split("; ");
        let mut game = Game {
            id: game_number,
            rounds: vec![],
        };
        for round in rounds {
            let mut r = Round{
                blue: 0,
                red: 0,
                green: 0,
            };
            for cube in round.split(", ") {
                let mut cube_info = cube.split(' ');
                let count: usize = match cube_info.next() {
                    Some(n) => n.parse().unwrap(),
                    None => panic!("no number found")
                };
                match cube_info.next() {
                    Some("red") => r.red = count,
                    Some("blue") => r.blue = count,
                    Some("green") => r.green = count,
                    Some(_) | None => panic!("invalid color")
                };
            }
            game.rounds.push(r)
        }
        games.push(game);
    }

    // now we can answer questions
    let q1 = Round{blue: 14, red: 12, green: 13};
    let filtered_games: Vec<Game> = games.into_iter().filter(|g| g.possible_for(&q1) ).collect();
    let sum: usize = filtered_games.into_iter().map(|g| g.id).sum();
    println!("Sum of possible games: {}", sum);
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>
}

#[derive(Debug)]
struct Round {
    blue: usize,
    red: usize,
    green: usize,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {}\n{:?}", self.id, self.rounds)
    }
}

impl Game {
    fn possible_for(&self, round: &Round) -> bool {
        for r in self.rounds.iter() {
            if r.blue > round.blue || r.green > round.green || r.red > round.red {
                return false;
            }
        }
        return true;
    }
}

use std::{cmp::Ordering, collections::HashMap};


// Wrong guesses
// - 248857578
fn main() {
    let input = input::get_input_lines();
    let lines = input.lines();

    let mut hands = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let hand: Hand = match &parts[..] {
            [hand, bet] => {
                let cards = hand.chars().map(|c| Card{sign:c}).collect::<Vec<Card>>();
                Hand::new(cards, bet.parse().unwrap())
            },
            _ => panic!("unknown line")
        };
        hands.push(hand);
    }

    let mut ordered = hands.clone();
    ordered.sort();

    let mut rank = ordered.len();
    let mut score = 0;
    for hand in ordered {
        score += hand.bid * rank;
        rank -= 1
    }
    println!("Score = {}", score);
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    cards: Vec<Card>,
    // possible should not be part of Hand
    bid: usize, 
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        Self {
            cards,
            bid,
        }
    }

    // returns it's type and an Option for the Highcard
    fn get_type(&self) -> (Type, Option<Card>) {
        // let mut ordered_cards = self.cards.clone();
        // ordered_cards.sort_by(|a, b| b.value().cmp(&a.value()));

        let mut map: HashMap<char, usize> = HashMap::new();
        for card in self.cards.iter() {
            map.entry(card.sign).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut saw_three = false;
        let mut saw_two = 0;
        for (char, count) in map {
            if count == 5 {
                return (Type::FiveOfAKind, None);
            }

            if count == 4 {
                return (Type::FourOfAKind, None);
            }

            if count == 3 {
                saw_three = true;
            }

            if count == 2 {
                saw_two += 1;
            }
        }

        if saw_three && saw_two == 1 {
            return (Type::FullHouse, None);
        }

        if saw_three {
            return (Type::ThreeOfAKind, None);
        }

        if saw_two == 2 {
            return (Type::TwoPair, None);
        }

        if saw_two == 1 {
            return (Type::OnePair, None);
        }

        let mut sorted = self.cards.clone();
        sorted.sort_by(|a,b| b.value().cmp(&a.value()));

        (Type::HighCard, sorted.first().cloned())
    }

    fn to_string(&self) -> String {
        self.cards.iter().map(|c|c.sign).collect::<String>()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.partial_cmp(other) == Some(Ordering::Equal);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_type() > other.get_type() {
            return Some(Ordering::Greater);
        }

        if self.get_type() < other.get_type() {
            return Some(Ordering::Less);
        }

        let zipped = self.cards.iter().zip(other.cards.iter());

        for (a, b) in zipped {
            if a.value() != b.value() {
                return Some(b.value().cmp(&a.value()));
            }
        }

        return Some(Ordering::Equal);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
struct Card {
    sign: char,
}

impl Card {
    fn value(&self) -> usize {
        match self.sign {
            c if ('1'..='9').contains(&c) => c.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => panic!("unknown card {}", c)
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

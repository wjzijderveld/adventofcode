use std::{cmp::Ordering, collections::HashMap};


// Wrong guesses part 1
// - 248857578
//
// Part 2:
// - 248829743 - too high
// - 248303660 - too low
// - 248820566 - too high
// - 248438364 = no indication
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
        println!("Hand={} Type={:?}", hand.to_string(), hand.get_type());
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

    // returns it's type
    fn get_type(&self) -> Type {
        let mut face_count: HashMap<char, usize> = HashMap::new();
        for card in self.cards.iter() {
            face_count.entry(card.sign).and_modify(|c| *c += 1).or_insert(1);
        }

        let m = face_count.clone();
        let joker_count = *m.get(&'J').or(Some(&0)).unwrap();

        let mut saw_four = false;
        let mut saw_three = false;
        let mut saw_twos: usize = 0;
        for (&sign, &count) in face_count.iter() {
            if count == 5 {
                // never gonna get better than this
                return Type::FiveOfAKind;
            }

            saw_four = saw_four || count == 4;
            saw_three = saw_three || count == 3;
            if count == 2 {
                saw_twos += 1;
            }
        }

        // 4+1 => jokers can be either the 4 or the leftover
        if saw_four {
            if joker_count > 0 {
                return Type::FiveOfAKind;
            }

            return Type::FourOfAKind;
        }


        if saw_three {
            if saw_twos > 0 {
                // if either the two or the three were all jokers we can make a FiveOfAKind
                if (2..=3).contains(&joker_count) {
                    return Type::FiveOfAKind;
                }
                return Type::FullHouse;
            }

            // if the three are jokers, we can make a FourOfAKind (FiveOfAKind was already checked)
            if joker_count == 3 {
                return Type::FourOfAKind;
            }

            // if there's jokers and it's not the three itself, we can make a full house
            if joker_count > 0 {
                return Type::FullHouse;
            }

            return Type::ThreeOfAKind;
        }

        if saw_twos > 0 {
            if saw_twos == 2 && joker_count > 0  {
                return Type::FullHouse;
            }

            if joker_count > 0 {
                return Type::ThreeOfAKind;
            }

            if saw_twos == 2 {
                return Type::TwoPair;
            }

            return Type::OnePair;
        }

        if joker_count > 0 {
            return Type::OnePair;
        }

        Type::HighCard
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
        // debug_cmp(self.clone(), other.clone());
        if self.get_type() > other.get_type() {
            return Some(Ordering::Greater);
        }

        if self.get_type() < other.get_type() {
            return Some(Ordering::Less);
        }

        let zipped = self.cards.iter().zip(other.cards.iter());

        for (sz, oz) in zipped {
            if sz.value() != oz.value() {
                return Some(oz.value().cmp(&sz.value()));
            }
        }

        return Some(Ordering::Equal);
    }
}

#[allow(dead_code)]
fn debug_cmp(s: Hand, o: Hand) {
    let ss = s.to_string();
    let os = o.to_string();

    if ss != "QQQJA" && ss != "T55J5" {
        return;
    }
    if os != "QQQJA" && os != "T55J5" {
        return;
    }

    dbg!(ss, os, s.get_type() > o.get_type());

    let zipped = s.cards.iter().zip(o.cards.iter());

    for (sz, oz) in zipped {
        if sz.value() != oz.value() {
            dbg!(sz.sign, oz.sign, oz.value().cmp(&sz.value()));
            return;
        }
    }
    dbg!(Ordering::Equal);
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
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => panic!("unknown card {}", c)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

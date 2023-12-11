use std::cmp::Ordering;

use crate::parser;

pub fn answer() {
    match part1(true) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
    match part2(false) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bet: u32,
    strength: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength > other.strength {
            return Ordering::Greater;
        }

        if self.strength < other.strength {
            return Ordering::Less;
        }

        if self.strength == other.strength {
            let self_cards = self.cards.chars().collect::<Vec<char>>();
            let other_cards = other.cards.chars().collect::<Vec<char>>();

            for n in 0..5 {
                if self_cards[n] == other_cards[n] {
                    continue;
                }

                if get_card_value(self_cards[n]) > get_card_value(other_cards[n]) {
                    return Ordering::Greater;
                }

                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day7/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day7/test.txt")?;
    }

    let mut hands = input
        .iter()
        .map(|x| {
            let mut split = x.split(" ");
            let cards = split.next().unwrap().to_string();
            let bet = split.next().unwrap().parse().unwrap();
            Hand {
                cards: cards.clone(),
                bet,
                strength: calculate_strength(cards.clone(), false),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bet * (i as u32 + 1))
        .sum::<u32>();

    println!("Part 1: {winnings}");

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day7/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day7/test.txt")?;
    }

    let mut hands = input
        .iter()
        .map(|x| {
            let mut split = x.split(" ");
            let cards = split.next().unwrap().to_string();
            let bet = split.next().unwrap().parse().unwrap();
            Hand {
                cards: cards.clone(),
                bet,
                strength: calculate_strength(cards.clone(), true),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bet * (i as u32 + 1))
        .sum::<u32>();

    println!("Part 2: {winnings}");

    Ok(())
}

fn calculate_strength(cards: String, _jokers: bool) -> u32 {
    let matches = cards
        .chars()
        .map(|c| {
            let mut matches = 0;
            for other in cards.chars() {
                if c == other {
                    matches += 1;
                }
            }
            matches
        })
        .collect::<Vec<u32>>();

    let max_matches = matches.iter().max().unwrap();
    let mut strength = 0;

    // High card
    if max_matches == &1 {
        strength = 1;
    }
    // One pair
    if max_matches == &2 {
        strength = 2;
    }
    // Two pair
    if max_matches == &2 && matches.iter().filter(|x| *x == &2).count() == 4 {
        strength = 3;
    }
    // Three of a kind
    if max_matches == &3 {
        strength = 4;
    }
    // Full house
    if max_matches == &3 && matches.iter().filter(|x| *x == &2).count() == 2 {
        strength = 5;
    }
    // Four of a kind
    if max_matches == &4 {
        strength = 6;
    }
    // Five of a kind
    if max_matches == &5 {
        strength = 7;
    }

    strength
}

fn get_card_value(card: char) -> u32 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

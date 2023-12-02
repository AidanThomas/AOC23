use std::collections::HashMap;

use crate::parser;

pub fn answer() {
    match part1(true) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
    match part2(true) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}

#[derive(Debug)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day2/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day2/test.txt")?;
    }

    let games: HashMap<u32, Vec<Set>> = input
        .iter()
        .map(|x| {
            let mut split = x.split(":");
            (
                split
                    .next()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                split
                    .next()
                    .unwrap()
                    .split(";")
                    .map(|x| x.split(",").map(|x| x.to_string()).collect::<Vec<String>>())
                    .map(|x| parse_to_set(x))
                    .collect(),
            )
        })
        .collect();

    let possible: u32 = games
        .iter()
        .filter(|(_, s)| {
            for set in *s {
                if !set.is_possible() {
                    return false;
                }
            }
            true
        })
        .map(|(g, _)| g.clone())
        .sum::<u32>();

    println!("Part 1: {0}", possible);

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day2/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day2/test.txt")?;
    }

    let games: HashMap<u32, Vec<Set>> = input
        .iter()
        .map(|x| {
            let mut split = x.split(":");
            (
                split
                    .next()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                split
                    .next()
                    .unwrap()
                    .split(";")
                    .map(|x| x.split(",").map(|x| x.to_string()).collect::<Vec<String>>())
                    .map(|x| parse_to_set(x))
                    .collect(),
            )
        })
        .collect();

    let powers = games.iter().fold(0, |acc, (_, s)| {
        acc + (s.iter().map(|x| x.blue).max().unwrap()
            * s.iter().map(|x| x.red).max().unwrap()
            * s.iter().map(|x| x.green).max().unwrap())
    });

    println!("Part 2: {:?}", powers);

    Ok(())
}

fn parse_to_set(set: Vec<String>) -> Set {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    for colour in set {
        let split: Vec<&str> = colour.trim().split(" ").collect();
        match split[1] {
            "blue" => blue = split[0].parse().unwrap(),
            "red" => red = split[0].parse().unwrap(),
            "green" => green = split[0].parse().unwrap(),
            _ => panic!("colour not recognised"),
        }
    }
    Set { blue, red, green }
}

impl Set {
    fn is_possible(&self) -> bool {
        return self.blue <= 14 && self.red <= 12 && self.green <= 13;
    }
}

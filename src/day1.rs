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

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day1/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day1/test.txt")?;
    }

    let mut values = Vec::<u32>::new();
    for line in input {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| match c.to_digit(10) {
                None => false,
                Some(_) => true,
            })
            .collect();

        let number = format!("{0}{1}", digits.first().unwrap(), digits.last().unwrap());
        values.push(match number.parse() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        });
    }

    println!("Part 1: {0}", values.iter().sum::<u32>());

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day1/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day1/test.txt")?;
    }

    let digit_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut values = Vec::<u32>::new();
    for line in input {
        let mut digits = Vec::<char>::new();
        let mut string = String::from("");
        for c in line.chars() {
            // Check if it is a digit
            match c.to_digit(10) {
                None => (),
                Some(_) => {
                    digits.push(c);
                    string = String::from("");
                }
            }

            // If not a digit, append to string and check for word
            string.push(c);
            for (k, v) in digit_map.iter() {
                if string.contains(k) {
                    digits.push(v.clone());
                    string = String::from("");
                    string.push(c);
                    break;
                }
            }
        }

        let number = format!("{0}{1}", digits.first().unwrap(), digits.last().unwrap());
        values.push(match number.parse() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        });
    }

    println!("Part 2: {0}", values.iter().sum::<u32>());

    Ok(())
}

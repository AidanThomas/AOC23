use crate::parser;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
    copies: u32,
}

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
        input = parser::read_by_line("inputs/day4/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day4/test.txt")?;
    }

    let cards: Vec<Card> = input
        .iter()
        .map(|x| {
            let mut split = x.split(":").last().unwrap().split("|");
            Card {
                winning: split
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
                actual: split
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
                copies: 1,
            }
        })
        .collect();

    let points = cards.iter().map(|v| v.calculate_points()).sum::<u32>();

    println!("Part 1: {:?}", points);

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day4/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day4/test.txt")?;
    }

    let mut cards: Vec<Card> = input
        .iter()
        .map(|x| {
            let mut split = x.split(":").last().unwrap().split("|");
            Card {
                winning: split
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
                actual: split
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| *x != "")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
                copies: 1,
            }
        })
        .collect();

    for i in 0..cards.len() {
        let copies = cards.get(i).unwrap().copies;
        let matches = cards[i].get_matches();

        // For each copy add 1 copy to next cards
        for j in 0..matches {
            let index = i as u32 + j + 1;
            cards.get_mut(index as usize).unwrap().copies += copies;
        }
    }

    let number_of_cards = cards.iter().map(|c| c.copies).sum::<u32>();

    println!("Part 2: {:?}", number_of_cards);

    Ok(())
}

impl Card {
    fn calculate_points(&self) -> u32 {
        let matches = self.get_matches();
        if matches == 0 {
            return 0;
        }
        return u32::pow(2, matches - 1);
    }

    fn get_matches(&self) -> u32 {
        self.actual
            .iter()
            .filter(|x| self.winning.contains(*x))
            .count() as u32
    }
}

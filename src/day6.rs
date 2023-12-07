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

struct Race {
    time: u64,
    distance: u64,
}

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day6/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day6/test.txt")?;
    }

    let input = input
        .iter()
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| *x != "")
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut races = Vec::<Race>::new();
    for i in 0..input[0].len() {
        races.push(Race {
            time: input[0][i],
            distance: input[1][i],
        })
    }

    let result = races.iter().map(|x| x.find_ways_to_beat()).product::<u64>();

    println!("Part 1: {:?}", result);
    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day6/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day6/test.txt")?;
    }

    let input = input
        .iter()
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .trim()
                .chars()
                .filter(|x| *x != ' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    let race = Race {
        time: input[0],
        distance: input[1],
    };

    println!("Part 2: {:?}", race.find_ways_to_beat());

    Ok(())
}

impl Race {
    fn find_ways_to_beat(&self) -> u64 {
        (0..=self.time)
            .map(|time_pressed| time_pressed * (self.time - time_pressed))
            .filter(|x| x > &self.distance)
            .count()
            .try_into()
            .unwrap()
    }
}

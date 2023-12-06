use crate::parser;

pub fn answer() {
    match part1(false) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
    match part2(false) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}

#[derive(Debug, Clone)]
struct Range {
    source: u64,
    destination: u64,
    range: u64,
}

#[derive(Debug, Clone)]
struct ConversionMap {
    ranges: Vec<Range>,
}

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day5/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day5/test.txt")?;
    }

    let mut lines = input.iter();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    lines.next();

    let mut conversion_maps = Vec::<ConversionMap>::new();
    let mut map = ConversionMap { ranges: Vec::new() };
    for line in lines {
        if line.contains("map") {
            continue;
        }
        if line != "" {
            let numbers: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            map.ranges.push(Range {
                source: numbers[1].clone(),
                destination: numbers[0].clone(),
                range: numbers[2].clone(),
            });
            continue;
        }
        conversion_maps.push(map.clone());
        map.ranges = Vec::new();
    }
    conversion_maps.push(map.clone());

    let locations = seeds
        .iter()
        .map(|s| {
            let mut number = *s;
            for map in &conversion_maps {
                number = map.convert(number);
            }
            number
        })
        .min()
        .unwrap();

    println!("Part 1: {:?}", locations);

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    // let input: Vec<String>;
    // if real {
    //     input = parser::read_by_line("inputs/day5/real.txt")?;
    // } else {
    //     input = parser::read_by_line("inputs/day5/test.txt")?;
    // }

    if real {
        println!("Part 2: ");
    }
    Ok(())
}

impl ConversionMap {
    fn convert(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if source >= range.source && source < range.source + range.range {
                return (source - range.source) + range.destination;
            }
        }

        return source;
    }
}

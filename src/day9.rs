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
        input = parser::read_by_line("inputs/day9/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day9/test.txt")?;
    }

    let histories: Vec<Vec<i32>> = input
        .iter()
        .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut extrapolated = Vec::<i32>::new();
    for history in &histories {
        let mut all_zero = false;
        let mut differences = vec![history.clone()];
        while !all_zero {
            let last = differences.last().unwrap();
            let mut new: Vec<i32> = last
                .iter()
                .enumerate()
                .map(|(i, v)| match last.get(i + 1) {
                    Some(x) => x - v,
                    None => 0,
                })
                .collect();
            new.remove(new.len() - 1);
            all_zero = new.iter().all(|x| *x == 0);
            differences.push(new);
        }

        let mut next = 0;
        differences.reverse();
        for d in differences.iter().skip(1) {
            let initial = d.last().unwrap();
            next = initial + next;
        }
        extrapolated.push(next);
    }

    println!("Part 1: {:?}", extrapolated.iter().sum::<i32>());

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day9/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day9/test.txt")?;
    }

    let histories: Vec<Vec<i32>> = input
        .iter()
        .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut extrapolated = Vec::<i32>::new();
    for history in &histories {
        let mut all_zero = false;
        let mut differences = vec![history.clone()];
        while !all_zero {
            let last = differences.last().unwrap();
            let mut new: Vec<i32> = last
                .iter()
                .enumerate()
                .map(|(i, v)| match last.get(i + 1) {
                    Some(x) => x - v,
                    None => 0,
                })
                .collect();
            new.remove(new.len() - 1);
            all_zero = new.iter().all(|x| *x == 0);
            differences.push(new);
        }

        let mut next = 0;
        differences.reverse();
        for d in differences.iter().skip(1) {
            let initial = d.first().unwrap();
            next = initial - next;
        }
        extrapolated.push(next);
    }

    println!("Part 2: {:?}", extrapolated.iter().sum::<i32>());

    Ok(())
}

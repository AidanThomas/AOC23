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

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day1/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day1/test.txt")?;
    }
    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day1/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day1/test.txt")?;
    }
    Ok(())
}

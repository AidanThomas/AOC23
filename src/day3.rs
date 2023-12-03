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
        input = parser::read_by_line("inputs/day3/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day3/test.txt")?;
    }

    let grid: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let mut numbers = Vec::<(i32, i32, i32, i32)>::new();
    let mut symbols = Vec::<(char, i32, i32)>::new();
    let mut current_number = "".to_string();
    let mut building_number = false;
    let mut number_start_x: i32 = 0;
    let mut number_end_x: i32;
    let mut number_y: i32 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c.to_digit(10) {
                Some(_) => {
                    current_number.push(c.clone());
                    if !building_number {
                        number_start_x = x as i32;
                        number_y = y as i32;
                        building_number = true;
                    }
                }
                None => {
                    if building_number {
                        number_end_x = x as i32 - 1;
                        if number_end_x < 0 {
                            number_end_x = (row.len() as i32) - 1;
                        }
                        numbers.push((
                            current_number.parse::<i32>().unwrap(),
                            number_start_x,
                            number_end_x,
                            number_y,
                        ));
                        current_number = "".to_string();
                        building_number = false;
                    }

                    if *c != '.' {
                        symbols.push((c.clone(), x as i32, y as i32));
                    }
                }
            }
        }
    }

    let parts: i32 = numbers
        .iter()
        .filter(|v| is_part(v, &symbols))
        .map(|v| v.0.clone())
        .sum();

    println!("Part 1: {:?}", parts);

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day3/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day3/test.txt")?;
    }

    let grid: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let mut numbers = Vec::<(i32, i32, i32, i32)>::new();
    let mut symbols = Vec::<(char, i32, i32)>::new();
    let mut current_number = "".to_string();
    let mut building_number = false;
    let mut number_start_x: i32 = 0;
    let mut number_end_x: i32;
    let mut number_y: i32 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c.to_digit(10) {
                Some(_) => {
                    current_number.push(c.clone());
                    if !building_number {
                        number_start_x = x as i32;
                        number_y = y as i32;
                        building_number = true;
                    }
                }
                None => {
                    if building_number {
                        number_end_x = x as i32 - 1;
                        if number_end_x < 0 {
                            number_end_x = (row.len() as i32) - 1;
                        }
                        numbers.push((
                            current_number.parse::<i32>().unwrap(),
                            number_start_x,
                            number_end_x,
                            number_y,
                        ));
                        current_number = "".to_string();
                        building_number = false;
                    }

                    if *c != '.' {
                        symbols.push((c.clone(), x as i32, y as i32));
                    }
                }
            }
        }
    }

    let parts: Vec<(i32, i32, i32, i32)> = numbers
        .iter()
        .filter(|v| is_part(v, &symbols))
        .map(|v| v.clone())
        .collect();

    let gear_ratios: i32 = symbols
        .iter()
        .filter(|v| v.0 == '*')
        .map(|v| get_gear_ratio(v, &parts))
        .sum();

    println!("Part 2: {:?}", gear_ratios);

    Ok(())
}

fn is_part(num_loc: &(i32, i32, i32, i32), symbols: &Vec<(char, i32, i32)>) -> bool {
    for s in symbols {
        if s.1 >= num_loc.1 - 1
            && s.1 <= num_loc.2 + 1
            && s.2 >= num_loc.3 - 1
            && s.2 <= num_loc.3 + 1
        {
            return true;
        }
    }
    return false;
}

fn get_gear_ratio(gear: &(char, i32, i32), parts: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let matches: Vec<i32> = parts
        .iter()
        .filter(|x| {
            gear.1 >= x.1 - 1 && gear.1 <= x.2 + 1 && gear.2 >= x.3 - 1 && gear.2 <= x.3 + 1
        })
        .map(|x| x.0)
        .collect();
    if matches.len() != 2 {
        return 0;
    }
    return matches[0] * matches[1];
}

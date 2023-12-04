use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = match args.get(1) {
        None => panic!("No day was specified"),
        Some(x) => x,
    };

    let parsed_day = match day.parse::<i32>() {
        Ok(x) => x,
        Err(_) => panic!("Please input a number"),
    };

    // Run code from individual days here
    match parsed_day {
        1 => day1::answer(),
        2 => day2::answer(),
        3 => day3::answer(),
        4 => day4::answer(),
        _ => (),
    }
}

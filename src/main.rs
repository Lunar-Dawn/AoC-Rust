use std::fs::read_to_string;
use std::process;

mod runner;
mod util;
mod year;

mod aoc2025;
mod dyn_result;

fn main() {
    let years = [(2025, aoc2025::solutions())];

    let args: Vec<String> = std::env::args().collect();

    let all_years = args.len() < 2;
    let requested_year = match args.get(1) {
        Some(year_str) => match year_str.parse::<usize>() {
            Ok(day) => day,
            Err(_) => {
                println!("Invalid day: {}", year_str);
                process::exit(1)
            }
        },
        None => 0,
    };

    let all_days = args.len() < 3;
    let requested_day = match args.get(2) {
        Some(day_str) => match day_str.parse::<usize>() {
            Ok(day) => day,
            Err(_) => {
                println!("Invalid day: {}", day_str);
                process::exit(1)
            }
        },
        None => 0,
    };

    for (y, solutions) in years {
        if !all_years && y != requested_year {
            continue;
        }

        for d in 0..solutions.len() {
            if all_days || requested_day == d + 1 {
                let input = read_to_string(format!("input/{y}/day{:02}.txt", d + 1)).unwrap();
                let (answer1, answer2) = solutions[d](&input);
                println!(
                    "Year {y}, Day {:02}:\n  Part 1: {answer1}\n  Part 2: {answer2}",
                    d + 1
                );
            }
        }
    }
}

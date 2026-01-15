use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::process;

use aoc_rust::util::Solution;
use aoc_rust::util::YearSolutions;

fn main() {
    let years = BTreeMap::from([
        (2023, aoc_rust::aoc2023::solutions()),
        (2024, aoc_rust::aoc2024::solutions()),
        (2025, aoc_rust::aoc2025::solutions()),
    ]);

    let args: Vec<String> = std::env::args().collect();

    let requested_year = args.get(1).map(|year_str| match year_str.parse::<usize>() {
        Ok(year) => year,
        Err(_) => {
            println!("Cannot parse year: {}", year_str);
            process::exit(1)
        }
    });

    let requested_day = args.get(2).map(|day_str| match day_str.parse::<usize>() {
        Ok(day) => day,
        Err(_) => {
            println!("Cannot parse day: {}", day_str);
            process::exit(1)
        }
    });

    if let Some(requested_year) = requested_year {
        let Some(year) = years.get(&requested_year) else {
            println!("Could not find year: {}", requested_year);
            process::exit(1);
        };

        if let Some(requested_day) = requested_day {
            let Some(runner) = year.get(&requested_day) else {
                println!("Could not find day: {}", requested_day);
                process::exit(1);
            };
            run_day(requested_year, requested_day, *runner);
        } else {
            run_all_in_year(requested_year, year)
        }
    } else {
        for year in years.iter() {
            run_all_in_year(*year.0, year.1)
        }
    }
}

fn run_all_in_year(year: usize, solutions: &YearSolutions) {
    for day in solutions {
        run_day(year, *day.0, *day.1);
    }
}
fn run_day(year: usize, day: usize, runner: Solution) {
    let input = read_to_string(format!("input/{year}/day{day:02}.txt")).unwrap();
    let (answer1, answer2) = runner(&input);
    println!("Year {year}, Day {day:02}:\n  Part 1: {answer1}\n  Part 2: {answer2}");
}

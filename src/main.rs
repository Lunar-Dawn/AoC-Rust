use std::path::Path;
use std::process;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn run(part1: util::Part, part2: util::Part, file_name: String) {
    let path = Path::new("input").join(file_name);
    match part1(&path) {
        Ok(result) => println!("Part 1: {}", result),
        Err(error) => println!("Part 1 Error: {}", error),
    }
    match part2(&path) {
        Ok(result) => println!("Part 2: {}", result),
        Err(error) => println!("Part 2 Error: {}", error),
    }
}

macro_rules! days {
    ($($x:ident),+) => {

        vec![
            $(
                ($x::part1, $x::part2),
            )+
        ]
    };
}

fn main() {
    let days: Vec<(util::Part, util::Part)> =
        days!(day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12);

    let args: Vec<String> = std::env::args().collect();

    let all = args.len() < 2;
    let day = match args.get(1) {
        Some(day_str) => match day_str.parse::<usize>() {
            Ok(day) => day,
            Err(_) => {
                println!("Invalid day: {}", day_str);
                process::exit(1)
            }
        },
        None => 0,
    };

    for i in 0..days.len() {
        if all || day == i + 1 {
            let (part1, part2) = days[i];
            run(part1, part2, format!("day{}.txt", i + 1));
        }
    }
}

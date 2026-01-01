use std::fs::read_to_string;
use std::process;

mod runner;
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
mod dyn_result;

macro_rules! days {
    ($($x:ident),+) => {
        vec![
            $(
                $x::run,
            )+
        ]
    };
}

fn main() {
    let days: Vec<_> =
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
            let input = read_to_string(format!("input/2025/day{:02}.txt", i + 1)).unwrap();
            let (answer1, answer2) = days[i](&input);
            println!("Part 1: {answer1}\nPart 2: {answer2}");
        }
    }
}

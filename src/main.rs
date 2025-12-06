use std::path::Path;
use std::process;

mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

fn main() {
    let days: [(util::Part, util::Part); _] = [
        (day1::part1, day1::part2),
        (day2::part1, day2::part2),
        (day3::part1, day3::part2),
        (day4::part1, day4::part2),
        (day5::part1, day5::part2),
    ];

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

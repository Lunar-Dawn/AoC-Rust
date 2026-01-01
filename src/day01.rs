use crate::dyn_result::DynResult;
use crate::runner;

runner!();

fn parse_line(line: &str) -> DynResult<i64> {
    let direction = match &line.chars().nth(0) {
        Some('L') => -1,
        _ => 1,
    };
    Ok(direction * &line[1..].parse::<i64>()?)
}
fn parse(input: &str) -> DynResult<Vec<i64>> {
    input.lines().map(parse_line).collect()
}

fn part1(instructions: &Vec<i64>) -> u64 {
    let mut total = 0;
    let mut position = 50;

    for instruction in instructions {
        position = (position + instruction).rem_euclid(100);
        if position == 0 {
            total += 1;
        }
    }

    total
}

fn part2(instructions: &Vec<i64>) -> u64 {
    let mut total = 0;
    let mut position = 50;

    for instruction in instructions {
        for _ in 0..instruction.abs() {
            position = (position + instruction.signum()).rem_euclid(100);

            if position == 0 {
                total += 1;
            }
        }
    }

    total
}

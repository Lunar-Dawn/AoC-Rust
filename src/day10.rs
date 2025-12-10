use crate::util;
use good_lp::{highs, variable, Expression, ProblemVariables, Solution, SolverModel};
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse_target_state(s: &str) -> u32 {
    let bits = s[1..s.len() - 1].chars().rev().map(|c| c == '#');

    let mut ret = 0;
    for b in bits {
        ret = (ret << 1) | b as u32;
    }
    ret
}
fn parse_operation(s: &str) -> u32 {
    let positions = s[1..s.len() - 1]
        .split(',')
        .map(|s| s.parse::<u64>().unwrap());

    let mut ret = 0;
    for pos in positions {
        ret = ret | (1 << pos);
    }
    ret
}
fn parse_taget_joltages(s: &str) -> Vec<u32> {
    s[1..s.len() - 1]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_line(s: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let parts: Vec<_> = s.split(' ').peekable().collect();

    let target_state = parse_target_state(parts[0]);

    let operations = parts[1..parts.len() - 1]
        .iter()
        .map(|s| parse_operation(s))
        .collect();

    let target_joltages = parse_taget_joltages(parts.last().unwrap());

    (target_state, operations, target_joltages)
}

fn try_buttons(goal: u32, buttons: &[u32], state: u32, num_pressed: u32) -> u32 {
    if state == goal {
        return num_pressed;
    }
    if buttons.len() == 0 {
        return u32::MAX;
    }

    let button = buttons[0];
    let buttons = &buttons[1..];

    min(
        try_buttons(goal, buttons, state, num_pressed),
        try_buttons(goal, buttons, state ^ button, num_pressed + 1),
    )
}
fn min_presses(goal: u32, buttons: &[u32]) -> u32 {
    try_buttons(goal, buttons, 0, 0)
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut total_presses = 0;
    for l in lines {
        let line = l?;

        let mut split = line.split(" ").peekable();

        let goal = parse_target_state(split.next().unwrap());
        let buttons: Vec<_> = split
            .by_ref()
            .take_while(|s| s.chars().next().unwrap() == '(')
            .map(parse_operation)
            .collect();

        total_presses += min_presses(goal, &buttons[..]);
    }

    Ok(total_presses.to_string())
}

pub(crate) fn solve_row(line: &str) -> u32 {
    let (_, operations, target_joltages) = parse_line(line);

    let mut vars = ProblemVariables::new();
    let button_presses: Vec<_> = operations
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let mut problem = vars
        .minimise(button_presses.iter().sum::<Expression>())
        .using(highs);

    for (target_index, &target) in target_joltages.iter().enumerate() {
        let mut expression = Expression::with_capacity(target_joltages.len());

        for (button_index, button) in operations.iter().enumerate() {
            if button & (1 << target_index) != 0 {
                expression += button_presses[button_index];
            }
        }

        problem.add_constraint(expression.eq(target as f64));
    }

    let solution = problem.solve().unwrap();
    button_presses
        .iter()
        .map(|v| solution.value(*v))
        .sum::<f64>() as u32
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let result: u32 = lines
        .map(|l| l.unwrap())
        .map(|l| solve_row(l.as_str()))
        .sum();

    Ok(result.to_string())
}

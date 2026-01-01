use std::cmp::min;

use good_lp::{highs, variable, Expression, ProblemVariables, Solution, SolverModel};

use crate::runner;
use crate::util::DynResult;

runner!();

struct Machine {
    target_state: u32,
    operations: Vec<u32>,
    target_joltages: Vec<u32>,
}
fn parse_target_state(s: &str) -> u32 {
    let bits = s[1..s.len() - 1].chars().rev().map(|c| c == '#');

    let mut ret = 0;
    for b in bits {
        ret = (ret << 1) | b as u32;
    }
    ret
}
fn parse_operation(s: &str) -> DynResult<u32> {
    let positions = s[1..s.len() - 1].split(',').map(|s| s.parse::<u64>());

    let mut ret = 0;
    for pos in positions {
        ret = ret | (1 << pos?);
    }
    Ok(ret)
}
fn parse_taget_joltages(s: &str) -> DynResult<Vec<u32>> {
    Ok(s[1..s.len() - 1]
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<_, _>>()?)
}

fn parse_line(s: &str) -> DynResult<Machine> {
    let parts: Vec<_> = s.split(' ').collect();

    let target_state = parse_target_state(parts[0]);

    let operations = parts[1..parts.len() - 1]
        .iter()
        .map(|s| parse_operation(s))
        .collect::<Result<_, _>>()?;

    let target_joltages = parse_taget_joltages(parts.last().unwrap())?;

    Ok(Machine {
        target_state,
        operations,
        target_joltages,
    })
}
fn parse(input: &str) -> DynResult<Vec<Machine>> {
    input.lines().map(parse_line).collect()
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
fn min_presses(machine: &Machine) -> u32 {
    try_buttons(machine.target_state, machine.operations.as_slice(), 0, 0)
}

fn part1(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(min_presses).sum()
}

fn solve_row(machine: &Machine) -> u32 {
    let mut vars = ProblemVariables::new();
    let button_presses: Vec<_> = machine
        .operations
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let mut problem = vars
        .minimise(button_presses.iter().sum::<Expression>())
        .using(highs);

    for (target_index, &target) in machine.target_joltages.iter().enumerate() {
        let mut expression = Expression::with_capacity(machine.target_joltages.len());

        for (button_index, button) in machine.operations.iter().enumerate() {
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

fn part2(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(solve_row).sum()
}

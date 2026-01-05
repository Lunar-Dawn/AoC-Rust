use crate::runner;
use crate::util::{parse, DynResult};

runner!();

type Problem = (u64, Vec<u64>);
type ParsedData = Vec<Problem>;
fn parse_line(line: &str) -> DynResult<Problem> {
    let mut parts = line.split(':');
    let goal = parts.next().unwrap().parse()?;
    let inputs = parse::parse_split_ws(parts.next().unwrap())?;

    Ok((goal, inputs))
}
fn parse(input: &str) -> DynResult<ParsedData> {
    input.lines().map(parse_line).collect()
}

fn is_valid_equation(goal: u64, inputs: &[u64], sum: u64) -> bool {
    if inputs.is_empty() {
        return sum == goal;
    }

    let next = inputs[0];
    is_valid_equation(goal, &inputs[1..], sum + next)
        || is_valid_equation(goal, &inputs[1..], sum * next)
}
fn part1(problems: &ParsedData) -> u64 {
    problems
        .iter()
        .filter(|(goal, inputs)| is_valid_equation(*goal, &inputs[1..], inputs[0]))
        .map(|(g, _)| g)
        .sum()
}

fn is_valid_equation_concat(goal: u64, inputs: &[u64], sum: u64) -> bool {
    if inputs.is_empty() {
        return sum == goal;
    }

    let next = inputs[0];

    // Slightly weird equation here, so here's the explanation
    // Concatenation is essentially left-shifting the sum and adding the rhs one.
    // But we can't left shift in base-10, so we calculate the next power of ten above
    // the rhs and multiply the sum by it.
    // ilog10 gives the number of base-10 digits in a number, so we add one and raise 10 to that power
    // to get the multiplier.
    let concat = (10u64.pow(next.ilog10() + 1)) * sum + next;

    is_valid_equation_concat(goal, &inputs[1..], sum + next)
        || is_valid_equation_concat(goal, &inputs[1..], sum * next)
        || is_valid_equation_concat(goal, &inputs[1..], concat)
}
fn part2(problems: &ParsedData) -> u64 {
    problems
        .iter()
        .filter(|(goal, inputs)| is_valid_equation_concat(*goal, &inputs[1..], inputs[0]))
        .map(|(g, _)| g)
        .sum()
}

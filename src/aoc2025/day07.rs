use crate::dyn_result::DynResult;
use crate::runner;

runner!();

fn parse(input: &str) -> DynResult<(usize, usize)> {
    let mut lines = input.lines();

    let Some(first_row) = lines.next() else {
        return Err("No lines found!".into());
    };
    let width = first_row.len();
    let Some(start_pos) = first_row.find('S') else {
        return Err("No start position found!".into());
    };

    let mut num_splits = 0;

    let mut state = vec![0; width];
    state[start_pos] = 1;

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if state[i] == 0 || c != '^' {
                continue;
            }

            num_splits += 1;

            state[i - 1] += state[i];
            state[i + 1] += state[i];
            state[i] = 0;
        }
    }

    Ok((num_splits, state.iter().sum()))
}

fn part1(result: &(usize, usize)) -> usize {
    result.0
}

fn part2(result: &(usize, usize)) -> usize {
    result.1
}

use crate::runner;
use crate::util::DynResult;

runner!();

fn parse(input: &str) -> DynResult<Vec<&str>> {
    Ok(input.lines().collect())
}

fn calculate_joltage(bank: &str, num_digits: usize) -> u64 {
    let mut total = 0;
    let mut start_pos = 0;

    for i in 0..num_digits {
        let (pos, value) = bank[start_pos..bank.len() - (num_digits - i - 1)]
            .bytes()
            .enumerate()
            .rev()
            .max_by_key(|&(_, c)| c)
            .unwrap();

        start_pos += pos + 1;
        total = total * 10 + value as u64 - ('0' as u64)
    }

    total
}

fn part1(banks: &Vec<&str>) -> u64 {
    banks.iter().map(|l| calculate_joltage(l, 2)).sum()
}

fn part2(banks: &Vec<&str>) -> u64 {
    banks.iter().map(|l| calculate_joltage(l, 12)).sum()
}

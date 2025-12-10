use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

fn calculate_joltage(bank: String, num_digits: usize) -> u64 {
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
        total = total * 10 + value as u64 - 0x30
    }

    total
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let jolts: u64 = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| calculate_joltage(l, 2))
        .sum();

    Ok(jolts.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let jolts: u64 = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| calculate_joltage(l, 12))
        .sum();

    Ok(jolts.to_string())
}

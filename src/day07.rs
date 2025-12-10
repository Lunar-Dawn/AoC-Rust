use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

pub fn solve(path: &PathBuf) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_row = lines.next().unwrap().unwrap();
    let width = first_row.len();
    let start_pos = first_row.find('S').unwrap();

    let mut num_splits = 0;

    let mut state = vec![0; width];
    state[start_pos] = 1;

    for line in lines {
        let line = line.unwrap();

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

    (num_splits, state.iter().sum())
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    Ok(solve(path).0.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    Ok(solve(path).1.to_string())
}

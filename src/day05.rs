use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

use crate::util;

fn parse_range(line: String) -> (u64, u64) {
    let num_strings = line
        .split("-")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (num_strings[0], num_strings[1])
}
fn read_ranges(lines: &mut Lines<BufReader<File>>) -> Vec<(u64, u64)> {
    let mut valid_ranges: Vec<_> = lines
        .by_ref()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .map(parse_range)
        .collect();
    valid_ranges.sort();

    let mut merged_ranges = Vec::new();

    let mut combined_range = valid_ranges[0];
    for range in valid_ranges[1..].iter() {
        if range.0 <= combined_range.1 + 1 {
            combined_range.1 = max(combined_range.1, range.1)
        } else {
            merged_ranges.push(combined_range);
            combined_range = *range;
        }
    }
    merged_ranges.push(combined_range);
    merged_ranges
}

fn is_fresh(id: u64, valid_ranges: &Vec<(u64, u64)>) -> bool {
    for (min, max) in valid_ranges {
        if *min <= id && id <= *max {
            return true;
        }
    }
    false
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut line_buffer = reader.lines();
    let valid_ranges = read_ranges(&mut line_buffer);

    let num_fresh = line_buffer
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .filter(|id| is_fresh(*id, &valid_ranges))
        .count();

    Ok(num_fresh.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut line_buffer = reader.lines();
    let valid_ranges = read_ranges(&mut line_buffer);

    let total = valid_ranges
        .into_iter()
        .map(|(min, max)| max - min + 1)
        .sum::<u64>();

    Ok(total.to_string())
}

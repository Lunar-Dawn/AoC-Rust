use std::path::PathBuf;

use crate::util;

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let input = std::fs::read_to_string(path)?.trim().to_string();

    let mut total = 0;

    for range in input.split(',') {
        let parts = range.split("-").collect::<Vec<&str>>();

        let lower = parts[0].parse::<u64>()?;
        let upper = parts[1].parse::<u64>()?;

        for id in lower..=upper {
            let divisor = 10_u64.pow((id.ilog10() + 1) / 2);

            if id / divisor == id % divisor {
                total += id;
            }
        }
    }

    Ok(total.to_string())
}

fn is_repeated(id: u64, pattern_length: u32) -> bool {
    let multiplier = 10_u64.pow(pattern_length);
    let pattern = id % multiplier;

    // Check if the pattern is 0 or has leading zeroes
    if pattern == 0 || pattern.ilog10() + 1 != pattern_length {
        return false;
    }

    let mut id_mut = id / multiplier;

    while id_mut > 0 {
        if id_mut % multiplier != pattern {
            return false;
        }
        id_mut /= multiplier;
    }

    true
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let input = std::fs::read_to_string(path)?.trim().to_string();

    let mut total = 0;

    for range in input.split(',') {
        let parts = range.split("-").collect::<Vec<&str>>();

        let lower = parts[0].parse::<u64>()?;
        let upper = parts[1].parse::<u64>()?;

        for id in lower..=upper {
            let max_length = (id.ilog10() + 1) / 2;

            for pattern_length in 1..=max_length {
                if is_repeated(id, pattern_length) {
                    total += id;
                    break;
                }
            }
        }
    }

    Ok(total.to_string())
}

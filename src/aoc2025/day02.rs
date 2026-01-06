use crate::runner;
use crate::util::numeric::{num_digits, split_digits};
use crate::util::{parse, DynResult};

runner!();

fn parse(input: &str) -> DynResult<Vec<(u64, u64)>> {
    input.trim().split(',').map(parse::parse_range).collect()
}

fn part1(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;

    for (lower, upper) in ranges {
        for id in *lower..=*upper {
            let [left, right] = split_digits(id);

            if left == right {
                total += id;
            }
        }
    }

    total
}

fn is_repeated(id: u64, pattern_length: u32) -> bool {
    let multiplier = 10_u64.pow(pattern_length);
    let pattern = id % multiplier;

    // Check if the pattern is 0 or has leading zeroes
    if pattern == 0 || num_digits(pattern) != pattern_length {
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

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;

    for (lower, upper) in ranges {
        for id in *lower..=*upper {
            let max_length = num_digits(id) / 2;

            for pattern_length in 1..=max_length {
                if is_repeated(id, pattern_length) {
                    total += id;
                    break;
                }
            }
        }
    }

    total
}

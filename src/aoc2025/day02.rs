use crate::runner;
use crate::util::{parse, DynResult};

runner!();

fn parse(input: &str) -> DynResult<Vec<(u64, u64)>> {
    input.trim().split(',').map(parse::parse_range).collect()
}

fn part1(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;

    for (lower, upper) in ranges {
        for id in *lower..=*upper {
            let divisor = 10_u64.pow((id.ilog10() + 1) / 2);

            if id / divisor == id % divisor {
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

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;

    for (lower, upper) in ranges {
        for id in *lower..=*upper {
            let max_length = (id.ilog10() + 1) / 2;

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

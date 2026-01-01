use std::cmp::max;

use crate::runner;
use crate::util;
use crate::util::DynResult;

runner!();

struct Input {
    fresh_ranges: Vec<(u64, u64)>,
    available_ingredients: Vec<u64>,
}
fn parse(input: &str) -> DynResult<Input> {
    let mut lines = input.lines();

    let raw_ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(util::parse_range)
        .collect::<Result<_, _>>()?;

    let fresh_ranges = combine_ranges(raw_ranges);
    let available_ingredients = lines.map(|l| l.parse()).collect::<Result<_, _>>()?;

    Ok(Input {
        fresh_ranges,
        available_ingredients,
    })
}

fn combine_ranges(mut raw_ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    raw_ranges.sort();

    let mut merged_ranges = Vec::new();

    let mut working_range = raw_ranges[0];
    for range in raw_ranges[1..].iter() {
        if range.0 <= working_range.1 + 1 {
            working_range.1 = max(working_range.1, range.1)
        } else {
            merged_ranges.push(working_range);
            working_range = *range;
        }
    }
    merged_ranges.push(working_range);
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

fn part1(input: &Input) -> usize {
    input
        .available_ingredients
        .iter()
        .filter(|id| is_fresh(**id, &input.fresh_ranges))
        .count()
}

fn part2(input: &Input) -> u64 {
    input
        .fresh_ranges
        .iter()
        .map(|(min, max)| max - min + 1)
        .sum()
}

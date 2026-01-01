use crate::runner;
use crate::util::DynResult;
use std::collections::HashMap;

runner!();

type ParsedData = (Vec<u64>, Vec<u64>);
fn parse_line(line: &str) -> DynResult<(u64, u64)> {
    let mut split = line.split_whitespace().map(|s| s.parse());

    let left = split.next().ok_or("No left element found")??;
    let right = split.next().ok_or("No left element found")??;

    Ok((left, right))
}
fn parse(input: &str) -> DynResult<ParsedData> {
    let pairs: Vec<_> = input.lines().map(parse_line).collect::<Result<_, _>>()?;

    Ok(pairs.into_iter().unzip())
}

fn part1((left, right): &ParsedData) -> u64 {
    let mut left = left.clone();
    left.sort();
    let mut right = right.clone();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}
fn part2((left, right): &ParsedData) -> u64 {
    let mut occurences = HashMap::new();
    for n in right {
        *occurences.entry(n).or_insert(0) += 1;
    }

    left.iter()
        .map(|n| n * occurences.get(n).unwrap_or(&0))
        .sum()
}

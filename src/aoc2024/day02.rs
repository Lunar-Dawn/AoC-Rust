use std::iter;

use crate::runner;
use crate::util::DynResult;

runner!();

// Deltas between adjacent numbers
type ParsedData = Vec<Vec<i64>>;
fn parse_line(line: &str) -> DynResult<Vec<i64>> {
    let values: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    Ok(values.windows(2).map(|w| w[1] - w[0]).collect())
}
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(input.lines().map(parse_line).collect::<Result<_, _>>()?)
}

fn is_valid(report: &[i64]) -> bool {
    if !report.windows(2).all(|w| w[0].signum() == w[1].signum()) {
        return false;
    }
    report.iter().map(|l| l.abs()).all(|l| l >= 1 && l <= 3)
}
fn part1(reports: &ParsedData) -> usize {
    reports.iter().filter(|r| is_valid(r)).count()
}

fn is_valid_with_skip(report: &Vec<i64>) -> bool {
    if is_valid(report) {
        return true;
    }
    if is_valid(&report[1..]) || is_valid(&report[..report.len() - 1]) {
        return true;
    }

    // I suspect that since the reports are so short the iterator creation overhead is slower
    // than the data shuffling of the previous version. But this was fun and experimenting more
    // with iterators is valuable.
    //
    // And if for some reason I want to analyze reports with 10'000 values this is likely faster.
    for i in 0..report.len() - 1 {
        let left = report[0..i].iter();
        let merged = report[i] + report[i + 1];
        let merged_it = iter::once(&merged);
        let right = report[i + 2..].iter();

        if left
            .chain(merged_it)
            .chain(right)
            .all(|l| l.signum() == merged.signum() && l.abs() >= 1 && l.abs() <= 3)
        {
            return true;
        }
    }

    false
}
fn part2(reports: &ParsedData) -> usize {
    reports.iter().filter(|r| is_valid_with_skip(r)).count()
}

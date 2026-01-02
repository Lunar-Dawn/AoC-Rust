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

    // You can definitely stick a couple of chains over slices together so not to do any data shuffling,
    // but I would have to measure if the performance even is better then.
    // The reports are all pretty short after all.
    let mut edited_report = vec![0; report.len() - 1];
    for i in 0..report.len() - 1 {
        for j in 0..i {
            edited_report[j] = report[j];
        }

        edited_report[i] = report[i] + report[i + 1];

        for j in (i + 1)..report.len() - 1 {
            edited_report[j] = report[j + 1];
        }

        if is_valid(&edited_report) {
            return true;
        }
    }

    false
}
fn part2(reports: &ParsedData) -> usize {
    reports.iter().filter(|r| is_valid_with_skip(r)).count()
}

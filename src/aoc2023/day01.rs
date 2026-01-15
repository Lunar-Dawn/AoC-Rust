use crate::runner;
use crate::util::numeric::prev_power_of_10;
use crate::util::parse::scan_integers;
use crate::util::DynResult;

runner!();

type ParsedData<'a> = Vec<&'a str>;
fn parse(input: &'_ str) -> DynResult<ParsedData<'_>> {
    Ok(input.lines().collect())
}

fn part1(lines: &ParsedData) -> u32 {
    lines
        .iter()
        .map(|line| {
            let numbers = scan_integers::<u32>(line);
            (numbers[0] / prev_power_of_10(numbers[0])) * 10 + numbers.last().unwrap() % 10
        })
        .sum()
}

fn matches(s: &str) -> Option<u32> {
    if let Some(n) = s.chars().next().map(|c| c.to_digit(10)).flatten() {
        return Some(n);
    }
    if s.starts_with("one") {
        return Some(1);
    }
    if s.starts_with("two") {
        return Some(2);
    }
    if s.starts_with("three") {
        return Some(3);
    }
    if s.starts_with("four") {
        return Some(4);
    }
    if s.starts_with("five") {
        return Some(5);
    }
    if s.starts_with("six") {
        return Some(6);
    }
    if s.starts_with("seven") {
        return Some(7);
    }
    if s.starts_with("eight") {
        return Some(8);
    }
    if s.starts_with("nine") {
        return Some(9);
    }

    None
}

fn search_front(line: &str) -> u32 {
    for i in 0..line.len() {
        if let Some(n) = matches(&line[i..]) {
            return n;
        }
    }

    unreachable!()
}
fn search_back(line: &str) -> u32 {
    for i in (0..line.len()).rev() {
        if let Some(n) = matches(&line[i..]) {
            return n;
        }
    }

    unreachable!()
}

fn part2(lines: &ParsedData) -> u32 {
    lines
        .iter()
        .map(|&line| search_front(line) * 10 + search_back(line))
        .sum()
}

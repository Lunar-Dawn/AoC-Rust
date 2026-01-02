use regex::Regex;

use crate::runner;
use crate::util::DynResult;

runner!();

type ParsedData<'a> = &'a str;
fn parse(input: &'_ str) -> DynResult<ParsedData<'_>> {
    Ok(input)
}

fn part1(memory: &ParsedData) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(memory)
        .map(|c| c.extract())
        .map(|(_, [l, r])| l.parse::<u64>().unwrap() * r.parse::<u64>().unwrap())
        .sum()
}
fn part2(memory: &ParsedData) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut result = 0;
    for capture in re.captures_iter(memory) {
        if &capture[0] == "do()" {
            enabled = true;
            continue;
        }
        if !enabled {
            continue;
        }
        if &capture[0] == "don't()" {
            enabled = false;
            continue;
        }

        let l: &u64 = &capture[1].parse().unwrap();
        let r: &u64 = &capture[2].parse().unwrap();
        result += l * r;
    }

    result
}

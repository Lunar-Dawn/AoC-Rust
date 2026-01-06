use std::collections::HashMap;

use crate::runner;
use crate::util::numeric::{num_digits, split_digits};
use crate::util::{parse, DynResult};

runner!();

fn num_successors(number: u64, rounds: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if rounds == 0 {
        return 1;
    }

    if let Some(successors) = cache.get(&(number, rounds)) {
        return *successors;
    }

    let result = if number == 0 {
        num_successors(1, rounds - 1, cache)
    } else if num_digits(number) % 2 == 0 {
        let [left, right] = split_digits(number);

        num_successors(left, rounds - 1, cache) + num_successors(right, rounds - 1, cache)
    } else {
        num_successors(number * 2024, rounds - 1, cache)
    };

    cache.insert((number, rounds), result);

    result
}

type ParsedData = (u64, u64);
fn parse(input: &str) -> DynResult<ParsedData> {
    let stones: Vec<u64> = parse::parse_split_ws(input)?;
    let mut cache = HashMap::new();

    Ok(stones
        .iter()
        .map(|s| {
            (
                num_successors(*s, 25, &mut cache),
                num_successors(*s, 75, &mut cache),
            )
        })
        .fold((0, 0), |l, r| (l.0 + r.0, l.1 + r.1)))
}

fn part1((ret, _): &ParsedData) -> u64 {
    *ret
}
fn part2((_, ret): &ParsedData) -> u64 {
    *ret
}

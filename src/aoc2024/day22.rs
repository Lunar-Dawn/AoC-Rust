use crate::runner;
use crate::util::DynResult;
use std::collections::BTreeMap;

runner!();

type ParsedData = Vec<u32>;
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?)
}

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret << 6;
    secret &= (1 << 24) - 1;
    secret ^= secret >> 5;
    secret &= (1 << 24) - 1;
    secret ^= secret << 11;
    secret &= (1 << 24) - 1;

    secret
}

fn part1(monkeys: &ParsedData) -> u64 {
    monkeys
        .iter()
        .map(|secret| {
            let mut secret = *secret;
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret as u64
        })
        .sum()
}

fn analyse_monkey(start: u32) -> BTreeMap<u32, i8> {
    let mut hash = 0;
    let mut prev;
    let mut curr = start;

    let mut sell_after = BTreeMap::new();

    for i in 0..2000 {
        prev = curr;
        curr = next_secret(curr);

        let price = (curr % 10) as i8;
        let change = price - (prev % 10) as i8;

        hash <<= 5;
        hash |= (change as u32) & 0b1_1111;
        hash &= (1 << 20) - 1;

        if i >= 3 {
            sell_after.entry(hash).or_insert(price);
        }
    }

    sell_after
}

fn part2(monkeys: &ParsedData) -> u64 {
    let mut sum_at = BTreeMap::new();

    for monkey in monkeys {
        let sells_at = analyse_monkey(*monkey);

        for (k, v) in sells_at.into_iter() {
            sum_at
                .entry(k)
                .and_modify(|p| *p += v as u64)
                .or_insert(v as u64);
        }
    }

    *sum_at.values().max().unwrap()
}

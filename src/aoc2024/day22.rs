use crate::runner;
use crate::util::DynResult;

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

const HASH_SIZE: usize = 1 << 20;

fn analyse_monkey(start: u32, total_price_after: &mut [u64], seen: &mut [i16], id: i16) {
    let mut hash = 0;
    let mut prev_price = (start % 10) as i8;
    let mut curr = start;

    for i in 0..2000 {
        curr = next_secret(curr);

        let price = (curr % 10) as i8;
        let change = price - prev_price;
        prev_price = price;

        hash <<= 5;
        hash |= (change as usize) & 0b1_1111;
        hash &= (1 << 20) - 1;

        if i >= 3 && seen[hash] != id {
            total_price_after[hash] += price as u64;
            seen[hash] = id;
        }
    }
}
fn part2(monkeys: &ParsedData) -> u64 {
    // Two big arrays to store state between monkeys
    let mut total_price_after = vec![0; HASH_SIZE];
    // Storing a bool array makes more "sense", but then you have to reset it.
    // Storing the last monkey to have seen the sequence means no resetting, since each run
    // can just check "was it this one?"
    let mut seen = vec![-1; HASH_SIZE];

    for i in 0..monkeys.len() {
        analyse_monkey(monkeys[i], &mut total_price_after, &mut seen, i as i16);
    }

    *total_price_after.iter().max().unwrap()
}

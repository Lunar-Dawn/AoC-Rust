use std::collections::BTreeSet;

use crate::runner;
use crate::util::parse::scan_integers;
use crate::util::DynResult;

runner!();

struct Card {
    winning_numbers: BTreeSet<u64>,
    your_numbers: BTreeSet<u64>,
}
impl Card {
    fn parse(line: &str) -> Self {
        let mut split = line.split(&[':', '|']).skip(1);

        let winning_str = split.next().unwrap();
        let your_str = split.next().unwrap();

        Self {
            winning_numbers: BTreeSet::from_iter(scan_integers(winning_str).into_iter()),
            your_numbers: BTreeSet::from_iter(scan_integers(your_str).into_iter()),
        }
    }

    fn matching_numbers(&self) -> usize {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .count()
    }
}

type ParsedData = Vec<Card>;
fn parse(input: &str) -> DynResult<ParsedData> {
    Ok(input.lines().map(Card::parse).collect())
}

fn part1(cards: &ParsedData) -> u64 {
    cards
        .iter()
        .map(|card| card.matching_numbers())
        .filter(|&n| n > 0)
        .map(|n| 2u64.pow(n as u32 - 1))
        .sum()
}
fn part2(cards: &ParsedData) -> u64 {
    let mut num_cards = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.matching_numbers() {
            num_cards[i + j + 1] += num_cards[i];
        }
    }

    num_cards.iter().sum()
}

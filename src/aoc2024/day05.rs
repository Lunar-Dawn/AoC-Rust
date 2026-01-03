use std::collections::{HashMap, HashSet};

use crate::runner;
use crate::util::{parse, DynResult};

runner!();

struct Input {
    rules: HashMap<u64, HashSet<u64>>,
    updates: Vec<Vec<u64>>,
}
type ParsedData = Input;
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines();
    let rule_lines = lines.by_ref().take_while(|l| !l.is_empty());

    let mut rules = HashMap::new();
    for line in rule_lines {
        let parts: Vec<_> = parse::parse_split_char(line, '|')?;
        rules
            .entry(parts[0])
            .or_insert(HashSet::new())
            .insert(parts[1]);
    }

    let updates: Vec<_> = lines
        .map(|l| parse::parse_split_char(l, ','))
        .collect::<Result<_, _>>()?;

    Ok(Input { rules, updates })
}

fn is_update_compliant(update: &Vec<u64>, input: &Input) -> bool {
    let mut seen = HashSet::new();
    for page in update {
        if let Some(rules) = &input.rules.get(page) {
            if matches!(rules.intersection(&seen).next(), Some(_)) {
                return false;
            }
        }

        seen.insert(*page);
    }

    true
}

fn part1(input: &ParsedData) -> u64 {
    input
        .updates
        .iter()
        .filter(|u| is_update_compliant(u, input))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn ordering_pass(update: &mut Vec<u64>, input: &Input) -> bool {
    let mut seen = HashMap::new();
    for i in 0..update.len() {
        let page = update[i];

        seen.insert(page, i);

        if let Some(rules) = &input.rules.get(&page) {
            let Some(swap) = rules.iter().find(|r| seen.contains_key(*r)) else {
                continue;
            };

            update.swap(i, seen[swap]);

            return true;
        }
    }

    false
}
fn fix_ordering(update: &Vec<u64>, input: &Input) -> Vec<u64> {
    let mut update = update.clone();

    while ordering_pass(&mut update, input) {}

    update
}
fn part2(input: &ParsedData) -> u64 {
    input
        .updates
        .iter()
        .filter(|u| !is_update_compliant(u, input))
        .map(|u| fix_ordering(u, input))
        .map(|u| u[u.len() / 2])
        .sum()
}

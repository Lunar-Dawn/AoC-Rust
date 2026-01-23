use std::cmp::{max, min};
use std::ops::Range;

use crate::runner;
use crate::util::parse::{scan_integers, take_integers};
use crate::util::DynResult;

runner!();

type ParsedData = (Vec<i64>, Vec<Vec<(Range<i64>, i64)>>);
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines();

    let seeds = scan_integers(lines.next().unwrap());
    lines.next();

    let mut maps = Vec::new();

    while let Some(_) = lines.next() {
        let mut map = Vec::new();

        loop {
            let line = lines.next();

            if line.is_none_or(|s| s.is_empty()) {
                break;
            }
            let line = line.unwrap();

            let (_, [dest_start, src_start, len]) = take_integers(line)?;
            map.push((src_start..src_start + len, dest_start - src_start));
        }

        maps.push(map);
    }

    Ok((seeds, maps))
}

fn part1((seeds, maps): &ParsedData) -> i64 {
    let mut seeds = seeds.clone();

    for map in maps {
        for seed in &mut seeds {
            if let Some((_, offset)) = map.iter().find(|(range, _)| range.contains(seed)) {
                *seed += offset;
            };
        }
    }

    *seeds.iter().min().unwrap()
}

fn offset_range(range: Range<i64>, offset: i64) -> Range<i64> {
    range.start + offset..range.end + offset
}

fn part2((seeds, maps): &ParsedData) -> i64 {
    let mut seed_ranges: Vec<_> = seeds.as_chunks().0.iter().map(|&[l, r]| l..l + r).collect();

    for map in maps {
        let mut i = 0;
        while i < seed_ranges.len() {
            let seed_range = &mut seed_ranges[i];
            i += 1;

            let overlap = map
                .iter()
                .map(|(map_range, offset)| {
                    (
                        max(seed_range.start, map_range.start)..min(seed_range.end, map_range.end),
                        offset,
                    )
                })
                .find(|(overlap, _)| !overlap.is_empty());

            let Some((overlap, offset)) = overlap else {
                continue;
            };

            let before = seed_range.start..overlap.start;
            let after = overlap.end..seed_range.end;

            *seed_range = offset_range(overlap, *offset);
            if !before.is_empty() {
                seed_ranges.push(before);
            }
            if !after.is_empty() {
                seed_ranges.push(after);
            }
        }
    }

    seed_ranges.into_iter().map(|r| r.start).min().unwrap()
}

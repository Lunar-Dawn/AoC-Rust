use std::collections::{HashMap, VecDeque};

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::DynResult;

runner!();

type ParsedData = (u64, u64);
fn calc(map: &VectorGrid<u8>, start: Point2i) -> ParsedData {
    let mut to_explore = VecDeque::new();
    let mut paths_to = HashMap::new();

    to_explore.push_back(start);
    paths_to.insert(start, 1);

    let mut rating = 0;
    let mut score = 0;

    while let Some(pos) = to_explore.pop_front() {
        let height = match map.get(&pos) {
            Some(9) => {
                score += 1;
                rating += paths_to[&pos];
                continue;
            }
            Some(h) => h,
            None => continue,
        };

        for neighbour in pos.neighbours_cardinal() {
            let Some(neighbour_height) = map.get(&neighbour) else {
                continue;
            };
            if *neighbour_height != height + 1 {
                continue;
            }

            if !paths_to.contains_key(&neighbour) {
                to_explore.push_back(neighbour);
            }
            *paths_to.entry(neighbour).or_insert(0) += paths_to[&pos];
        }
    }

    (score, rating)
}
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let data = lines
        .into_iter()
        .flat_map(|l| l.bytes())
        .map(|c| c - b'0')
        .collect();

    let map = VectorGrid::from(width, height, data);

    Ok(map
        .pos_iter()
        .filter(|p| matches!(map.get(p), Some(0)))
        .map(|p| calc(&map, p))
        .reduce(|l, r| (l.0 + r.0, l.1 + r.1))
        .unwrap())
}
fn part1((ret, _): &ParsedData) -> u64 {
    *ret
}
fn part2((_, ret): &ParsedData) -> u64 {
    *ret
}

use std::collections::HashSet;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::pathfinding::dijkstra_best_paths;
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type ParsedData = (u64, usize);
fn parse(input: &str) -> DynResult<ParsedData> {
    let map = VectorGrid::from_str_vec(input.lines().collect());

    let start = Point2i::new(1, map.height() as i64 - 2);
    let end = Point2i::new(map.width() as i64 - 2, 1);
    let (cost, _, nodes_from) = dijkstra_best_paths(
        (start, Vec2i::RIGHT),
        |(pos, _)| *pos == end,
        |(pos, dir)| {
            let mut ret = Vec::new();

            if *map.get(&(pos + dir)).unwrap() != '#' {
                ret.push((1, (pos + dir, *dir)));
            }
            ret.push((1000, (*pos, dir.turn_clockwise())));
            ret.push((1000, (*pos, dir.turn_anticlockwise())));

            ret
        },
    )
    .unwrap();

    let on_best_path: HashSet<_> = nodes_from.into_iter().map(|((pos, _), _)| pos).collect();

    Ok((cost, on_best_path.len()))
}

fn part1((ret, _): &ParsedData) -> u64 {
    *ret
}
fn part2((_, ret): &ParsedData) -> usize {
    *ret
}

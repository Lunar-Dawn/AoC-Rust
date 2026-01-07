use std::collections::HashSet;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::pathfinding::dijkstra;
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type ParsedData = (u64, usize);
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let data = lines.iter().flat_map(|line| line.chars()).collect();

    let map = VectorGrid::from(width, height, data);

    let start = Point2i::new(1, map.height() as i64 - 2);
    let end = Point2i::new(map.width() as i64 - 2, 1);
    let (cost, _, nodes_from) = dijkstra(
        (start, Vec2i::RIGHT),
        |(pos, dir)| {
            let mut ret = Vec::new();

            if *map.get(&(pos + dir)).unwrap() != '#' {
                ret.push((1, (pos + dir, *dir)));
            }
            ret.push((1000, (*pos, dir.turn_clockwise())));
            ret.push((1000, (*pos, dir.turn_anticlockwise())));

            ret
        },
        |(pos, _)| *pos == end,
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

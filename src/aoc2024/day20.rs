use std::collections::VecDeque;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type ParsedData = (VectorGrid<char>, Point2i);
fn parse(input: &str) -> DynResult<ParsedData> {
    let map = VectorGrid::from_str_vec(input.lines().collect());
    let end = map
        .pos_iter()
        .find(|p| *map.get(p).unwrap() == 'E')
        .unwrap();

    Ok((map, end))
}

fn build_distance_from(map: &VectorGrid<char>, origin: &Point2i) -> VectorGrid<Option<u64>> {
    let mut distance_from = VectorGrid::new(map.width(), map.height(), None);

    let mut to_visit = VecDeque::new();
    to_visit.push_back((*origin, 0));

    while let Some((pos, distance)) = to_visit.pop_front() {
        for neighbour_pos in pos.neighbours_cardinal() {
            if *map.get(&neighbour_pos).unwrap() == '#' {
                continue;
            }

            if distance_from.get(&neighbour_pos).unwrap().is_none() {
                to_visit.push_back((neighbour_pos, distance + 1));
            }
        }

        *distance_from.get_mut(&pos).unwrap() = Some(distance);
    }

    distance_from
}

fn num_cheats_within(
    map: &VectorGrid<char>,
    end_distances: &VectorGrid<Option<u64>>,
    origin: Point2i,
    max_dist: i64,
    threshold: u64,
) -> u64 {
    let mut cheats = 0;
    let origin_cost = end_distances.get(&origin).unwrap().unwrap();

    for y in 0..=max_dist {
        for x in -max_dist..=max_dist {
            if y == 0 && x > 0 {
                continue;
            }

            let p = origin + Vec2i::new(x, y);
            let dist = origin.distance_manhattan(&p);

            if dist > max_dist {
                continue;
            }

            if map.get(&p).is_none_or(|c| c == &'#') {
                continue;
            };

            let cheat_end_cost = end_distances.get(&p).unwrap().unwrap();

            let savings = origin_cost.abs_diff(cheat_end_cost) - dist as u64;

            if savings >= threshold {
                cheats += 1;
            }
        }
    }

    cheats
}

fn part1((map, end): &ParsedData) -> u64 {
    let end_distances = build_distance_from(map, &end);

    let mut num_big_saves = 0;

    for pos in map.pos_iter() {
        if *map.get(&pos).unwrap() == '#' {
            continue;
        }

        num_big_saves += num_cheats_within(&map, &end_distances, pos, 2, 100);
    }

    num_big_saves
}
fn part2((map, end): &ParsedData) -> u64 {
    let end_distances = build_distance_from(map, &end);

    let mut num_big_saves = 0;

    for pos in map.pos_iter() {
        if *map.get(&pos).unwrap() == '#' {
            continue;
        }

        num_big_saves += num_cheats_within(&map, &end_distances, pos, 20, 100);
    }
    num_big_saves
}

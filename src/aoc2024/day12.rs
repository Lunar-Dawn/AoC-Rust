use std::collections::{HashSet, VecDeque};

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type Region = HashSet<Point2i>;

fn parse_region(
    plots: &VectorGrid<char>,
    visited: &mut VectorGrid<bool>,
    start: Point2i,
) -> Option<Region> {
    if *visited.get(&start).unwrap() {
        return None;
    }

    let mut region = HashSet::new();
    let crop = plots.get(&start).unwrap();

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while let Some(pos) = to_visit.pop_front() {
        match visited.get_mut(&pos) {
            Some(true) | None => continue,
            Some(s) => *s = true,
        }

        region.insert(pos);

        for neighbour_pos in pos.neighbours_cardinal() {
            let Some(neighbour) = plots.get(&neighbour_pos) else {
                continue;
            };

            if neighbour != crop {
                continue;
            }

            if *visited.get(&neighbour_pos).unwrap() {
                continue;
            }

            to_visit.push_back(neighbour_pos);
        }
    }

    Some(region)
}

type ParsedData = Vec<Region>;
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let data = lines.iter().flat_map(|line| line.chars()).collect();
    let plots = VectorGrid::from(width, height, data);
    let mut visited = VectorGrid::new(plots.width(), plots.height(), false);

    Ok(plots
        .pos_iter()
        .filter_map(|pos| parse_region(&plots, &mut visited, pos))
        .collect())
}

fn area(region: &Region) -> usize {
    region.len()
}
fn perimeter(region: &Region) -> usize {
    let mut ret = 0;

    for pos in region {
        for neighbour in pos.neighbours_cardinal() {
            if !region.contains(&neighbour) {
                ret += 1;
            }
        }
    }

    ret
}
fn part1(regions: &ParsedData) -> usize {
    regions.iter().map(|r| area(r) * perimeter(r)).sum()
}
fn sides(region: &Region) -> usize {
    // In a polygon the number of vertices equals the number of edges,
    // so just count the number of corners.
    // Outer corners are easy, look at adjacent neighbours
    // and if neither are in the region it's a corner.
    // Inner corners check that they are both in the region but also need to check that
    // the diagonal between them isn't

    let mut corners = 0;

    for pos in region {
        for i in 0..4 {
            let offset = Vec2i::DIRECTIONS_CARDINAL[i];
            let neighbour = region.contains(&(pos + offset));
            let neighbour_cw = region.contains(&(pos + offset.turn_clockwise()));

            if neighbour != neighbour_cw {
                continue;
            }

            if !neighbour || !region.contains(&(pos + offset + offset.turn_clockwise())) {
                corners += 1;
            }
        }
    }

    corners
}
fn part2(regions: &ParsedData) -> usize {
    regions.iter().map(|r| area(r) * sides(r)).sum()
}

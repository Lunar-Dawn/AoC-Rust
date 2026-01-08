use std::collections::HashMap;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::parse::take_integers;
use crate::util::pathfinding::{astar_best_cost, astar_best_path};
use crate::util::point2i::Point2i;
use crate::util::DynResult;

runner!();

const GRID_SIZE: usize = 71;
const PART_1_NUM_OBSTACLES: u64 = 1024;

type ParsedData = (VectorGrid<u64>, Vec<Point2i>);

fn parse(input: &str) -> DynResult<ParsedData> {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let (_, [x, y]) = take_integers(l).unwrap();
            Point2i::new(x, y)
        })
        .collect();

    let mut map = VectorGrid::new(GRID_SIZE, GRID_SIZE, u64::MAX);
    for (i, p) in points.iter().enumerate() {
        *map.get_mut(&p).unwrap() = i as u64;
    }

    Ok((map, points))
}

fn get_valid_neighbours<'a>(
    point: &'a Point2i,
    map: &'a VectorGrid<u64>,
    current_time: u64,
) -> impl Iterator<Item = (u64, Point2i)> + use<'a> {
    point
        .neighbours_cardinal()
        .filter_map(move |n| map.get(&n).map(|i| (*i, n)))
        .filter(move |(arrival_time, _)| current_time < *arrival_time)
}

fn neighbours(point: &Point2i, map: &VectorGrid<u64>, current_time: u64) -> Vec<(u64, Point2i)> {
    get_valid_neighbours(point, map, current_time)
        .map(|(_, n)| (1, n))
        .collect()
}
// Calculates the valid neighbours with a single twist, points where corruption is going to fall
// have a higher cost the *sooner* it will arrive. Thus, the path will avoid points where
// corruption falls as much as possible, hopefully well enough to avoid needing to redo
// the pathfinding too many times.
fn valid_neighbours(
    point: &Point2i,
    map: &VectorGrid<u64>,
    current_time: u64,
    max_time: u64,
) -> Vec<(u64, Point2i)> {
    get_valid_neighbours(point, map, current_time)
        .map(|(i, n)| {
            if i == u64::MAX {
                (1, n)
            } else {
                let cost = (max_time - i) * (GRID_SIZE * GRID_SIZE) as u64;
                (cost, n)
            }
        })
        .collect()
}

fn part1((map, _): &ParsedData) -> u64 {
    let end = Point2i::new(GRID_SIZE as i64 - 1, GRID_SIZE as i64 - 1);
    astar_best_cost(
        Point2i::new(0, 0),
        end,
        |p| neighbours(p, map, PART_1_NUM_OBSTACLES),
        |p| p.distance_manhattan(&end) as u64,
    )
    .unwrap()
}
fn part2((map, points): &ParsedData) -> Point2i {
    let end = Point2i::new(GRID_SIZE as i64 - 1, GRID_SIZE as i64 - 1);
    let mut path_taken = HashMap::new();

    // A bit of a hack to make sure it always run the first time.
    path_taken.insert(points[PART_1_NUM_OBSTACLES as usize], None);

    for i in PART_1_NUM_OBSTACLES.. {
        if path_taken.contains_key(&points[i as usize]) {
            let path = astar_best_path(
                Point2i::new(0, 0),
                end,
                |p| valid_neighbours(p, map, i, points.len() as u64),
                |p| p.distance_manhattan(&end) as u64,
            );
            let Some((_, _, new_path)) = path else {
                return points[i as usize];
            };

            path_taken = new_path;
        }
    }
    unreachable!("The path was never blocked")
}

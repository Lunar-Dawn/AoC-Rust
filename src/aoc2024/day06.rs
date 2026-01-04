use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;
use std::collections::HashSet;

runner!();

type ParsedData = (usize, usize);

fn find_start(lines: &Vec<&str>) -> Result<Point2i, &'static str> {
    for y in 0..lines.len() {
        for (x, c) in lines[y].chars().enumerate() {
            if c == '^' {
                return Ok(Point2i::new(x as i64, y as i64));
            }
        }
    }
    Err("No starting position found")
}
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    let start = find_start(&lines)?;

    let data = lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c == '#'))
        .collect();

    let grid = VectorGrid::from(width, height, data);

    Ok(simulate(&grid, start))
}

// I am not happy with the code repetition here, but it'd be very thorny to deduplicate
fn simulate(grid: &VectorGrid<bool>, mut pos: Point2i) -> (usize, usize) {
    let mut visited = HashSet::new();
    let mut dir_i = 0;

    let mut blocking_obstacles = 0;

    loop {
        visited.insert(pos);

        let next_pos = pos + Vec2i::DIRECTIONS_CARDINAL[dir_i];

        match grid.get(&next_pos) {
            Some(true) => dir_i = (dir_i + 1) % 4,
            Some(false) => {
                if !visited.contains(&next_pos) && try_place_obstacle(&grid, pos, dir_i) {
                    blocking_obstacles += 1;
                }

                pos = next_pos
            }
            None => break,
        }
    }

    (visited.len(), blocking_obstacles)
}
fn try_place_obstacle(grid: &VectorGrid<bool>, mut pos: Point2i, mut dir_i: usize) -> bool {
    let obstacle_pos = pos + Vec2i::DIRECTIONS_CARDINAL[dir_i];

    dir_i = (dir_i + 1) % 4;

    let mut visited = HashSet::new();
    loop {
        if !grid.in_bounds(&pos) {
            return false;
        }

        let next_pos = pos + Vec2i::DIRECTIONS_CARDINAL[dir_i];
        if next_pos == obstacle_pos || matches!(grid.get(&next_pos), Some(true)) {
            if visited.contains(&(pos, dir_i)) {
                return true;
            }

            visited.insert((pos, dir_i));

            dir_i = (dir_i + 1) % 4;
        } else {
            pos = next_pos;
        }
    }
}

fn part1((ret, _): &ParsedData) -> usize {
    *ret
}
fn part2((_, ret): &ParsedData) -> usize {
    *ret
}

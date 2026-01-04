use std::collections::{HashSet, VecDeque};
use std::ops::DerefMut;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

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

type WorkerQueue = (Mutex<(VecDeque<(Point2i, usize)>, bool)>, Condvar);
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    let start = find_start(&lines)?;

    let data = lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c == '#'))
        .collect();

    let grid = Arc::new(VectorGrid::from(width, height, data));

    let queue = Arc::new((Mutex::new((VecDeque::new(), false)), Condvar::new()));

    let mut threads = Vec::new();

    for _ in 0..4 {
        let grid = grid.clone();
        let queue = queue.clone();
        threads.push(thread::spawn(move || obstacle_worker(grid, queue)))
    }

    let part1 = simulate(&grid, start, &queue);
    let part2 = threads.into_iter().map(|t| t.join().unwrap()).sum();

    Ok((part1, part2))
}

fn obstacle_worker(grid: Arc<VectorGrid<bool>>, queue: Arc<WorkerQueue>) -> usize {
    let cond = &queue.1;
    let queue = &queue.0;

    let mut result = 0;

    loop {
        let (pos, dir) = match cond
            .wait_while(queue.lock().unwrap(), |(q, kill)| {
                if q.is_empty() {
                    !*kill
                } else {
                    false
                }
            })
            .unwrap()
            .deref_mut()
        {
            (q, true) => {
                if q.is_empty() {
                    break;
                } else {
                    q.pop_front().unwrap()
                }
            }
            (q, false) => q.pop_front().unwrap(),
        };

        if try_place_obstacle(&grid, pos, dir) {
            result += 1;
        }
    }

    result
}

// I am not happy with the code repetition here, but it'd be very thorny to deduplicate
fn simulate(grid: &VectorGrid<bool>, mut pos: Point2i, queue: &Arc<WorkerQueue>) -> usize {
    let mut visited = HashSet::new();
    let mut dir_i = 0;

    loop {
        visited.insert(pos);

        let next_pos = pos + Vec2i::DIRECTIONS_CARDINAL[dir_i];

        match grid.get(&next_pos) {
            Some(true) => dir_i = (dir_i + 1) % 4,
            Some(false) => {
                if !visited.contains(&next_pos) {
                    queue.0.lock().unwrap().0.push_back((pos, dir_i));
                    queue.1.notify_one();
                }

                pos = next_pos
            }
            None => break,
        }
    }

    queue.0.lock().unwrap().1 = true;
    queue.1.notify_all();

    visited.len()
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

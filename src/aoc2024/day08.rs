use std::collections::{HashMap, HashSet};

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::DynResult;

runner!();

struct Map {
    grid: VectorGrid<char>,
    antennae: HashMap<char, Vec<Point2i>>,
}
type ParsedData = Map;
fn parse(input: &str) -> DynResult<ParsedData> {
    let grid = VectorGrid::from_str_vec(input.lines().collect());

    let mut antennae = HashMap::new();
    for p in grid.pos_iter() {
        let c = grid.get(&p).unwrap();

        if *c == '.' {
            continue;
        }

        antennae.entry(*c).or_insert_with(Vec::new).push(p);
    }

    Ok(Map { grid, antennae })
}

fn part1(map: &ParsedData) -> usize {
    let mut antinodes = HashSet::new();

    for (_, antennae) in map.antennae.iter() {
        for from in antennae {
            for to in antennae {
                if from == to {
                    continue;
                }

                let antinode = from + to.vec_to(from);
                if map.grid.in_bounds(&antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}
fn part2(map: &ParsedData) -> usize {
    let mut antinodes = HashSet::new();

    for (_, antennae) in map.antennae.iter() {
        for from in antennae {
            for to in antennae {
                if from == to {
                    continue;
                }

                let displacement = from.vec_to(to);
                for i in 1.. {
                    let antinode = from + displacement * i;

                    if !map.grid.in_bounds(&antinode) {
                        break;
                    }

                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::util;

struct UnionFind {
    parent: Vec<usize>,
}
impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        self.parent[node] = self.find(self.parent[node]);
        self.parent[node]
    }
    fn union(&mut self, node1: usize, node2: usize) {
        let p = self.find(node1);
        self.parent[p] = self.find(node2);
    }
}

type JunctionPos = (u64, u64, u64);

fn parse_position(string: &str) -> JunctionPos {
    let offsets: Vec<_> = string.split(',').map(|s| s.parse().unwrap()).collect();

    (offsets[0], offsets[1], offsets[2])
}

fn distance_sq(p1: JunctionPos, p2: JunctionPos) -> u64 {
    p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2) + p1.2.abs_diff(p2.2).pow(2)
}

fn parse_input(path: &PathBuf) -> Vec<JunctionPos> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    lines
        .map(|l| l.unwrap())
        .map(|l| parse_position(l.as_str()))
        .collect()
}
fn calculate_junction_distances(
    junctions: &Vec<JunctionPos>,
) -> BinaryHeap<Reverse<(u64, (usize, usize))>> {
    (0..junctions.len())
        .flat_map(|x| std::iter::repeat(x).zip((x + 1)..junctions.len()))
        .map(|p| (distance_sq(junctions[p.0], junctions[p.1]), p))
        .map(Reverse)
        .collect::<BinaryHeap<_>>()
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let junctions: Vec<_> = parse_input(path);
    let mut closest_distances = calculate_junction_distances(&junctions);

    let mut circuits = UnionFind::new(junctions.len());

    for _ in 0..1000 {
        let Reverse((_, (p1, p2))) = closest_distances.pop().unwrap();

        circuits.union(p1, p2);
    }

    let mut sizes = vec![0; junctions.len()];

    for i in 0..junctions.len() {
        let p = circuits.find(i);
        sizes[p] += 1;
    }
    sizes.sort();
    let result: u64 = sizes.iter().rev().take(3).product();

    Ok(result.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let junctions: Vec<_> = parse_input(path);
    let mut closest_distances = calculate_junction_distances(&junctions);

    let mut circuits = UnionFind::new(junctions.len());

    let mut result = 0;

    loop {
        let Some(Reverse((_, (p1, p2)))) = closest_distances.pop() else {
            break;
        };

        if circuits.find(p1) == circuits.find(p2) {
            continue;
        }

        circuits.union(p1, p2);
        result = junctions[p1].0 * junctions[p2].0;
    }

    Ok(result.to_string())
}

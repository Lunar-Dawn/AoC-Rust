use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::runner;
use crate::util::DynResult;

runner!();

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

fn parse_position(string: &str) -> DynResult<JunctionPos> {
    let offsets: Vec<_> = string
        .split(',')
        .map(|s| s.parse::<u64>())
        .collect::<Result<_, _>>()?;

    Ok((offsets[0], offsets[1], offsets[2]))
}
fn parse(input: &str) -> DynResult<(u64, u64)> {
    let junctions: Vec<_> = input
        .lines()
        .map(parse_position)
        .collect::<Result<_, _>>()?;

    let mut closest_distances = calculate_junction_distances(&junctions);
    let mut circuits = UnionFind::new(junctions.len());

    for _ in 0..1000 {
        let Some(Reverse((_, (p1, p2)))) = closest_distances.pop() else {
            return Err("Not enough connections to do part 1".into());
        };

        circuits.union(p1, p2);
    }

    let mut sizes = vec![0; junctions.len()];
    for i in 0..junctions.len() {
        let p = circuits.find(i);
        sizes[p] += 1;
    }
    sizes.sort();
    let part1 = sizes.iter().rev().take(3).product();

    let mut part2 = 0;
    loop {
        let Some(Reverse((_, (p1, p2)))) = closest_distances.pop() else {
            break;
        };

        if circuits.find(p1) == circuits.find(p2) {
            continue;
        }

        circuits.union(p1, p2);
        part2 = junctions[p1].0 * junctions[p2].0;
    }

    Ok((part1, part2))
}

fn distance_sq(p1: JunctionPos, p2: JunctionPos) -> u64 {
    p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2) + p1.2.abs_diff(p2.2).pow(2)
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

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}

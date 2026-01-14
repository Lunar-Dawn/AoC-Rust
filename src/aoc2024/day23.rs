use std::collections::{HashMap, HashSet};

use crate::runner;
use crate::util::DynResult;

runner!();

struct Computer<'a> {
    name: &'a str,
    neighbours: HashSet<&'a str>,
}
impl<'a> Computer<'a> {
    fn new(name: &'a str) -> Self {
        Computer {
            name,
            neighbours: HashSet::new(),
        }
    }
    fn connect(&mut self, to: &'a str) {
        self.neighbours.insert(to);
    }

    fn triangles(&self, set: &mut HashSet<[&'a str; 3]>, computers: &ParsedData) {
        for n1 in self.neighbours.iter() {
            for n2 in self.neighbours.iter() {
                if n1 == n2 {
                    continue;
                }

                if !computers[n1].neighbours.contains(n2) {
                    continue;
                }

                let mut triangle = [self.name, n1, n2];
                triangle.sort();
                set.insert(triangle);
            }
        }
    }
}

type ParsedData<'a> = HashMap<&'a str, Computer<'a>>;
fn parse(input: &'_ str) -> DynResult<ParsedData<'_>> {
    let mut computers = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split("-");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();

        computers
            .entry(from)
            .or_insert(Computer::new(from))
            .connect(to);
        computers
            .entry(to)
            .or_insert(Computer::new(to))
            .connect(from);
    }

    Ok(computers)
}

fn bron_kerbosch<'a>(
    mut r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    max_clique: &mut (usize, Vec<&'a str>),
    computers: &ParsedData<'a>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.0 {
            *max_clique = (r.len(), r.into_iter().collect());
        }
        return;
    }

    let pivot = p
        .union(&x)
        .max_by_key(|name| computers[*name].neighbours.len())
        .unwrap();
    let pivot_neighbours = &computers[pivot].neighbours;
    let candidates: Vec<_> = p.difference(pivot_neighbours).copied().collect();

    for name in candidates {
        r.insert(name);
        let neighbours = &computers[name].neighbours;
        let p_restricted = p.intersection(neighbours).copied().collect();
        let x_restricted = x.intersection(neighbours).copied().collect();

        bron_kerbosch(r.clone(), p_restricted, x_restricted, max_clique, computers);

        p.remove(name);
        x.insert(name);

        r.remove(name);
    }
}

fn part1(computers: &ParsedData) -> usize {
    let mut triangles = HashSet::new();

    for computer in computers.values() {
        computer.triangles(&mut triangles, computers);
    }

    let valid: Vec<_> = triangles
        .iter()
        .filter(|t| t.iter().any(|c| c.starts_with('t')))
        .collect();

    valid.len()
}
fn part2(computers: &ParsedData) -> String {
    let r = HashSet::new();
    let p = computers.keys().copied().collect();
    let x = HashSet::new();

    let mut max_clique = (0, Vec::new());
    bron_kerbosch(r, p, x, &mut max_clique, computers);

    max_clique.1.sort();

    max_clique.1.join(",")
}

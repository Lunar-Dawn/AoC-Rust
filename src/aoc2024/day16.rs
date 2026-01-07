use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::runner;
use crate::util::grid::{Grid, VectorGrid};
use crate::util::point2i::Point2i;
use crate::util::vec2i::Vec2i;
use crate::util::DynResult;

runner!();

type PFNode = (Point2i, Vec2i);
struct RaceTrack {
    map: VectorGrid<char>,
    start: Point2i,
    end: Point2i,

    to_visit: BinaryHeap<Reverse<(u64, PFNode)>>,
    visited: HashMap<PFNode, (u64, Vec<PFNode>)>,
}
impl RaceTrack {
    fn new(map: VectorGrid<char>) -> Self {
        let start = Point2i::new(1, map.height() as i64 - 2);
        let end = Point2i::new(map.width() as i64 - 2, 1);
        RaceTrack {
            map,
            start,
            end,
            to_visit: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }

    fn is_wall(&self, p: &Point2i) -> bool {
        *self.map.get(p).unwrap() == '#'
    }
    fn add_node(&mut self, cost: u64, from: PFNode, to: PFNode) {
        let Some((prev_cost, from)) = self.visited.get_mut(&from) else {
            self.to_visit.push(Reverse((cost, from)));
            self.visited.insert(from, (cost, Vec::from([to])));

            return;
        };
        if *prev_cost == cost {
            from.push(to);
        } else if *prev_cost > cost {
            *prev_cost = cost;
            from.clear();
            from.push(to);
        }
    }

    fn find_max_paths(&self, best_cost: u64) -> HashSet<Point2i> {
        let mut to_backtrace = VecDeque::new();
        let mut on_max_path = HashSet::new();

        for d in Vec2i::DIRECTIONS_CARDINAL {
            let Some((c, _)) = self.visited.get(&(self.end, d)) else {
                continue;
            };

            if *c > best_cost {
                continue;
            }

            to_backtrace.push_back((self.end, d));
        }

        while let Some((p, d)) = to_backtrace.pop_front() {
            on_max_path.insert(p);

            let (_, from) = self.visited.get(&(p, d)).unwrap();
            for from in from {
                to_backtrace.push_back(*from);
            }
        }

        on_max_path
    }

    fn solve(mut self) -> (u64, usize) {
        self.to_visit.push(Reverse((0, (self.start, Vec2i::RIGHT))));
        self.visited
            .insert((self.start, Vec2i::RIGHT), (0, Vec::new()));

        let mut best_cost = None;

        while let Some(Reverse((cost, node))) = self.to_visit.pop() {
            if let Some(c) = best_cost {
                if c < cost {
                    break;
                }
            }

            if node.0 == self.end {
                best_cost = match best_cost {
                    None => Some(cost),
                    Some(c) => Some(c.min(cost)),
                };
            }

            let (pos, dir) = node;

            if !self.is_wall(&(pos + dir)) {
                self.add_node(cost + 1, (pos + dir, dir), node);
            }
            self.add_node(cost + 1000, (pos, dir.turn_clockwise()), node);
            self.add_node(cost + 1000, (pos, dir.turn_anticlockwise()), node);
        }

        let on_max_path = self.find_max_paths(best_cost.unwrap());

        for p in on_max_path.iter() {
            *self.map.get_mut(&p).unwrap() = 'O';
        }
        println!("{}", self.map);

        (best_cost.unwrap(), on_max_path.len())
    }
}

type ParsedData = (u64, usize);
fn parse(input: &str) -> DynResult<ParsedData> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let data = lines.iter().flat_map(|line| line.chars()).collect();

    let map = VectorGrid::from(width, height, data);

    Ok(RaceTrack::new(map).solve())
}

fn part1((ret, _): &ParsedData) -> u64 {
    *ret
}
fn part2((_, ret): &ParsedData) -> usize {
    *ret
}

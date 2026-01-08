use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use super::traits::PathfindingCache;

pub struct AllPathsCache<Node>
where
    Node: Eq + Clone + Hash,
{
    visited_from: HashMap<Node, (u64, Vec<Node>)>,
    end_nodes: Option<(u64, Vec<Node>)>,
}
impl<Node> AllPathsCache<Node>
where
    Node: Eq + Clone + Hash,
{
    pub fn new(start: Node) -> AllPathsCache<Node> {
        let mut visited_from = HashMap::new();
        visited_from.insert(start, (0, Vec::new()));
        Self {
            visited_from,
            end_nodes: None,
        }
    }
    pub fn trace_best_paths(self) -> Option<(u64, Vec<Node>, HashMap<Node, Vec<Node>>)> {
        let Some((cost, end_nodes)) = self.end_nodes else {
            return None;
        };

        let mut to_backtrace = VecDeque::from(end_nodes.clone());

        let mut came_from = HashMap::new();

        while let Some(n) = to_backtrace.pop_front() {
            if came_from.contains_key(&n) {
                continue;
            }

            let (_, arrived_from) = self.visited_from.get(&n).unwrap();

            for from in arrived_from.iter() {
                to_backtrace.push_back(from.clone());
            }
            came_from.entry(n).or_insert(arrived_from.clone());
        }

        Some((cost, end_nodes, came_from))
    }
}

impl<Node> PathfindingCache for AllPathsCache<Node>
where
    Node: Eq + Clone + Hash,
{
    type Node = Node;

    fn visit(&mut self, cost: u64, to: Self::Node, from: Self::Node) -> bool {
        let (prev_cost, prev) = self
            .visited_from
            .entry(to)
            .or_insert((u64::MAX, Vec::new()));
        if *prev_cost > cost {
            *prev_cost = cost;
            prev.clear();
            prev.push(from);
            true
        } else if *prev_cost == cost {
            prev.push(from);
            false
        } else {
            false
        }
    }

    fn finalise_path(&mut self, cost: u64, node: Self::Node) {
        let Some((prev_cost, set)) = self.end_nodes.as_mut() else {
            self.end_nodes = Some((cost, vec![node]));
            return;
        };

        assert_eq!(*prev_cost, cost, "Tried to insert new end node with different cost than existing, queue order must be messed up");

        set.push(node);
    }
    fn final_cost(&self) -> Option<u64> {
        self.end_nodes.as_ref().map(|(cost, _)| *cost)
    }

    fn should_search_all_ends(&self) -> bool {
        true
    }
}

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use super::traits::PathfindingCache;

pub struct BestPathCache<Node>
where
    Node: Eq + Clone + Hash,
{
    visited_from: HashMap<Node, (u64, Option<Node>)>,
    end_node: Option<(u64, Node)>,
}
impl<Node> BestPathCache<Node>
where
    Node: Eq + Clone + Hash,
{
    pub fn new(start: Node) -> BestPathCache<Node> {
        let mut visited_from = HashMap::new();
        visited_from.insert(start, (0, None));
        Self {
            visited_from,
            end_node: None,
        }
    }
    pub fn trace_best_path(self) -> Option<(u64, Node, HashMap<Node, Option<Node>>)> {
        let Some((cost, end_node)) = self.end_node else {
            return None;
        };

        let mut to_backtrace = VecDeque::new();
        to_backtrace.push_front(end_node.clone());

        let mut came_from = HashMap::new();

        while let Some(n) = to_backtrace.pop_front() {
            if came_from.contains_key(&n) {
                continue;
            }

            let (_, arrived_from) = self.visited_from.get(&n).unwrap();

            if let Some(arrived_from) = arrived_from {
                to_backtrace.push_back(arrived_from.clone());
            }

            came_from.entry(n).or_insert(arrived_from.clone());
        }

        Some((cost, end_node, came_from))
    }
}

impl<Node> PathfindingCache for BestPathCache<Node>
where
    Node: Eq + Clone + Hash,
{
    type Node = Node;

    fn visit(&mut self, cost: u64, to: Self::Node, from: Self::Node) -> bool {
        let (prev_cost, prev) = self.visited_from.entry(to).or_insert((u64::MAX, None));
        if *prev_cost > cost {
            *prev_cost = cost;
            *prev = Some(from);
            true
        } else {
            false
        }
    }

    fn finalise_path(&mut self, cost: u64, node: Self::Node) {
        self.end_node = Some((cost, node));
    }
    fn final_cost(&self) -> Option<u64> {
        self.end_node.as_ref().map(|(cost, _)| *cost)
    }

    fn should_search_all_ends(&self) -> bool {
        true
    }
}

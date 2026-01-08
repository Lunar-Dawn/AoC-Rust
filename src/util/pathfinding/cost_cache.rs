use crate::util::pathfinding::traits::PathfindingCache;
use std::collections::HashMap;
use std::hash::Hash;

pub struct CostCache<Node>
where
    Node: Eq + Hash,
{
    visited: HashMap<Node, u64>,
    final_cost: Option<u64>,
}

impl<Node> CostCache<Node>
where
    Node: Eq + Hash,
{
    pub fn new(start: Node) -> Self {
        let mut visited = HashMap::new();
        visited.insert(start, 0);
        Self {
            visited,
            final_cost: None,
        }
    }
}
impl<Node> PathfindingCache for CostCache<Node>
where
    Node: Eq + Hash,
{
    type Node = Node;

    fn visit(&mut self, cost: u64, to: Self::Node, _: Self::Node) -> bool {
        let prev_cost = self.visited.entry(to).or_insert(u64::MAX);
        if *prev_cost > cost {
            *prev_cost = cost;
            true
        } else {
            false
        }
    }
    fn finalise_path(&mut self, cost: u64, _: Self::Node) {
        match self.final_cost {
            None => self.final_cost = Some(cost),
            Some(prev_cost) => self.final_cost = Some(prev_cost.min(cost)),
        }
    }
    fn final_cost(&self) -> Option<u64> {
        self.final_cost
    }

    fn should_search_all_ends(&self) -> bool {
        false
    }
}

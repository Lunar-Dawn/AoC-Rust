use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::traits::PathfindingQueue;

pub struct DijkstraQueue<Node>
where
    Node: Ord,
{
    to_visit: BinaryHeap<Reverse<(u64, Node)>>,
}
impl<Node> DijkstraQueue<Node>
where
    Node: Ord,
{
    pub fn new(start: Node) -> Self {
        let mut to_visit = BinaryHeap::new();

        to_visit.push(Reverse((0, start)));

        Self { to_visit }
    }
}
impl<Node> PathfindingQueue for DijkstraQueue<Node>
where
    Node: Ord,
{
    type Node = Node;

    fn push(&mut self, cost: u64, to: Self::Node) {
        self.to_visit.push(Reverse((cost, to)));
    }
    fn pop(&mut self) -> Option<(u64, Self::Node)> {
        self.to_visit.pop().map(|Reverse(e)| e)
    }
}

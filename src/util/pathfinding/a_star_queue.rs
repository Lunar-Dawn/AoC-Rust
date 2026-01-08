use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::traits::PathfindingQueue;

// Needs a custom comparator so it sorts lowest estimate and then highest depth
#[derive(PartialEq, Eq)]
struct AStarHeapNode<Node: Ord>(u64, u64, Node);
impl<Node: Ord> PartialOrd<Self> for AStarHeapNode<Node> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl<Node: Ord> Ord for AStarHeapNode<Node> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => {}
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
        }
        match self.1.cmp(&other.1) {
            Ordering::Equal => self.2.cmp(&other.2),
            o => o,
        }
    }
}

pub struct AStarQueue<Node, Heuristic>
where
    Node: Ord,
    Heuristic: Fn(&Node) -> u64,
{
    to_visit: BinaryHeap<AStarHeapNode<Node>>,
    heuristic: Heuristic,
}
impl<Node, Heuristic> AStarQueue<Node, Heuristic>
where
    Node: Ord,
    Heuristic: Fn(&Node) -> u64,
{
    pub fn new(start: Node, heuristic: Heuristic) -> Self {
        let mut ret = Self {
            to_visit: BinaryHeap::new(),

            heuristic,
        };

        ret.to_visit.push(ret.heap_node(0, start));

        ret
    }

    fn heap_node(&self, cost: u64, node: Node) -> AStarHeapNode<Node> {
        AStarHeapNode(cost + (self.heuristic)(&node), cost, node)
    }
}
impl<Node, Heuristic> PathfindingQueue for AStarQueue<Node, Heuristic>
where
    Node: Ord,
    Heuristic: Fn(&Node) -> u64,
{
    type Node = Node;

    fn push(&mut self, cost: u64, to: Self::Node) {
        self.to_visit.push(self.heap_node(cost, to));
    }

    fn pop(&mut self) -> Option<(u64, Self::Node)> {
        self.to_visit.pop().map(|AStarHeapNode(_, c, n)| (c, n))
    }
}

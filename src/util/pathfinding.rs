use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

struct PFData<Node> {
    to_visit: BinaryHeap<Reverse<(u64, Node)>>,
    visited: HashMap<Node, (u64, Vec<Node>)>,

    end_nodes: Option<(u64, Vec<Node>)>,
}
impl<Node> PFData<Node>
where
    Node: Ord + Clone + Hash,
{
    fn new(start: Node) -> Self {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashMap::new();

        to_visit.push(Reverse((0, start.clone())));
        visited.insert(start, (0, Vec::new()));

        Self {
            to_visit,
            visited,
            end_nodes: None,
        }
    }

    fn push(&mut self, cost: u64, to: Node, from: Node) {
        if let Some((prev_cost, prev)) = self.visited.get_mut(&to) {
            if *prev_cost == cost {
                prev.push(from);
            } else if *prev_cost > cost {
                *prev_cost = cost;
                prev.clear();
                prev.push(from);
            }
        } else {
            self.visited.insert(to.clone(), (cost, Vec::from([from])));
            self.to_visit.push(Reverse((cost, to)));
        };
    }
    fn pop(&mut self) -> Option<(u64, Node)> {
        self.to_visit.pop().map(|Reverse(e)| e)
    }

    fn trace_best_paths(self) -> (Vec<Node>, HashMap<Node, Vec<Node>>) {
        let Some((_, end_nodes)) = self.end_nodes else {
            panic!("Attempted to retrieve best paths before calculating them")
        };

        let mut to_backtrace = VecDeque::from(end_nodes.clone());

        let mut came_from = HashMap::new();

        while let Some(n) = to_backtrace.pop_front() {
            let (_, arrived_from) = self.visited.get(&n).unwrap();

            for from in arrived_from.iter() {
                to_backtrace.push_back(from.clone());
            }
            came_from.entry(n).or_insert(arrived_from.clone());
        }

        (end_nodes, came_from)
    }

    fn add_end_node(&mut self, n: Node, c: u64) {
        let Some((prev_cost, set)) = self.end_nodes.as_mut() else {
            self.end_nodes = Some((c, vec![n]));
            return;
        };

        assert_eq!(*prev_cost, c, "Tried to insert new end node with different cost than existing, queue order must be messed up");

        set.push(n);
    }
    fn final_cost(&self) -> Option<u64> {
        self.end_nodes.as_ref().map(|(cost, _)| *cost)
    }
}

pub fn dijkstra<Node, NeighbourF, EndF>(
    start: Node,
    neighbours: NeighbourF,
    is_end: EndF,
) -> Option<(u64, Vec<Node>, HashMap<Node, Vec<Node>>)>
where
    Node: Ord + Clone + Hash,
    NeighbourF: Fn(&Node) -> Vec<(u64, Node)>,
    EndF: Fn(&Node) -> bool,
{
    let mut pf_data = PFData::new(start);

    while let Some((cost, node)) = pf_data.pop() {
        if let Some(final_cost) = pf_data.final_cost() {
            if final_cost < cost {
                break;
            }
        }

        // If there are multiple possible end nodes the loop can't simply break here. It needs to
        // explore all the remaining nodes with a cost equal to it. It's a bit of extra work
        // if the end node is unique, but it's likely fairly little.
        if is_end(&node) {
            pf_data.add_end_node(node, cost);
            continue;
        }

        for (c, n) in neighbours(&node) {
            pf_data.push(cost + c, n, node.clone());
        }
    }

    let Some(final_cost) = pf_data.final_cost() else {
        return None;
    };
    let (end_starting_points, nodes_from) = pf_data.trace_best_paths();

    Some((final_cost, end_starting_points, nodes_from))
}

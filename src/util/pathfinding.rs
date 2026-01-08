use std::collections::HashMap;
use std::hash::Hash;

mod a_star_queue;
mod all_paths_cache;
mod best_path_cache;
mod cost_cache;
mod dijkstra_queue;
mod traits;

use a_star_queue::AStarQueue;
use all_paths_cache::AllPathsCache;
use best_path_cache::BestPathCache;
use cost_cache::CostCache;
use dijkstra_queue::DijkstraQueue;
use traits::{PathfindingCache, PathfindingQueue};

// The structure here is somewhat overengineered, but it was a good lesson in traits, composition,
// and it makes mixing and matching trivial.

fn run_pathfinder<Node, NeighbourF, EndF, Queue, Cache>(
    mut queue: Queue,
    mut cache: Cache,
    is_end: EndF,
    neighbours: NeighbourF,
) -> Cache
where
    Node: Ord + Clone + Hash,
    NeighbourF: Fn(&Node) -> Vec<(u64, Node)>,
    EndF: Fn(&Node) -> bool,
    Queue: PathfindingQueue<Node = Node>,
    Cache: PathfindingCache<Node = Node>,
{
    while let Some((cost, node)) = queue.pop() {
        if let Some(final_cost) = cache.final_cost() {
            if final_cost < cost {
                break;
            }
        }

        if is_end(&node) {
            cache.finalise_path(cost, node);
            if cache.should_search_all_ends() {
                continue;
            } else {
                break;
            }
        }

        for (c, n) in neighbours(&node) {
            let cost = cost + c;
            if cache.visit(cost, n.clone(), node.clone()) {
                queue.push(cost, n);
            }
        }
    }
    cache
}

// Finds the lowest-cost path, but does not build it.
pub fn astar_best_cost<Node, NeighbourF, Heuristic>(
    start: Node,
    goal: Node,
    neighbours: NeighbourF,
    heuristic: Heuristic,
) -> Option<u64>
where
    Node: Ord + Clone + Hash,
    NeighbourF: Fn(&Node) -> Vec<(u64, Node)>,
    Heuristic: Fn(&Node) -> u64,
{
    let cache = run_pathfinder(
        AStarQueue::new(start.clone(), heuristic),
        CostCache::new(start),
        |n: &Node| *n == goal,
        neighbours,
    );
    cache.final_cost()
}
pub fn astar_best_path<Node, NeighbourF, Heuristic>(
    start: Node,
    goal: Node,
    neighbours: NeighbourF,
    heuristic: Heuristic,
) -> Option<(u64, Node, HashMap<Node, Option<Node>>)>
where
    Node: Ord + Clone + Hash,
    NeighbourF: Fn(&Node) -> Vec<(u64, Node)>,
    Heuristic: Fn(&Node) -> u64,
{
    let cache = run_pathfinder(
        AStarQueue::new(start.clone(), heuristic),
        BestPathCache::new(start),
        |n: &Node| *n == goal,
        neighbours,
    );
    cache.trace_best_path()
}

// Finds all the best paths to the end, and returns them all.
pub fn dijkstra_all_paths<Node, NeighbourF, EndF>(
    start: Node,
    is_end: EndF,
    neighbours: NeighbourF,
) -> Option<(u64, Vec<Node>, HashMap<Node, Vec<Node>>)>
where
    Node: Ord + Clone + Hash,
    NeighbourF: Fn(&Node) -> Vec<(u64, Node)>,
    EndF: Fn(&Node) -> bool,
{
    let cache = run_pathfinder(
        DijkstraQueue::new(start.clone()),
        AllPathsCache::new(start),
        is_end,
        neighbours,
    );
    cache.trace_best_paths()
}

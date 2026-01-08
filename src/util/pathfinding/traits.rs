pub trait PathfindingQueue {
    type Node;

    fn push(&mut self, cost: u64, to: Self::Node);
    fn pop(&mut self) -> Option<(u64, Self::Node)>;
}
pub trait PathfindingCache {
    type Node;

    fn visit(&mut self, cost: u64, to: Self::Node, from: Self::Node) -> bool;

    fn finalise_path(&mut self, cost: u64, node: Self::Node);
    fn final_cost(&self) -> Option<u64>;

    fn should_search_all_ends(&self) -> bool;
}

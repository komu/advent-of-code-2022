use std::cmp::Reverse;
use std::hash::Hash;

use hashbrown::HashMap;
use priority_queue::PriorityQueue;

pub trait Graph {
    type Node: Eq + Hash + Clone;

    fn is_solution(&self, node: &Self::Node) -> bool;
    fn collect_neighbors(&self, node: &Self::Node, neighbors: &mut Vec<(Self::Node, u32)>);
    fn heuristic_distance(&self, node: &Self::Node) -> u32;
}

pub fn shortest_path_len<G>(g: &G, start: G::Node) -> Option<(G::Node, u32)>
    where
        G: Graph,
{
    let mut g_score = HashMap::<G::Node, u32>::new();
    let mut open_set = PriorityQueue::<G::Node, Reverse<u32>>::new();
    let mut neighbors = Vec::new();

    g_score.insert(start.clone(), 0);
    let start_distance = g.heuristic_distance(&start);
    open_set.push(start, Reverse(start_distance));

    while let Some((current, _)) = open_set.pop() {
        let current_gscore = *g_score.get(&current).unwrap();

        if g.is_solution(&current) {
            return Some((current, current_gscore));
        }

        g.collect_neighbors(&current, &mut neighbors);
        for (neighbor, cost) in neighbors.drain(..) {
            let tentative_gscore = current_gscore + cost;
            if tentative_gscore < g_score.get(&neighbor).copied().unwrap_or(u32::max_value()) {
                g_score.insert(neighbor.clone(), tentative_gscore);

                let neighbor_score = tentative_gscore + g.heuristic_distance(&neighbor);
                open_set.push(neighbor, Reverse(neighbor_score));
            }
        }
    }

    None
}

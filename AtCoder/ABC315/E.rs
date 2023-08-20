use im_rc::HashSet;
use itertools::Itertools;
use proconio::*;
use proconio::marker::Usize1;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

fn main() {
  input! {
    n: usize
  }

  let mut graph = vec![];
  let mut edges = vec![vec![]; n];

  for i in 0..n {
    input! {
        c: usize,
        p: [Usize1; c]
    }
    edges[i] = p.clone();
    for start in p {
        graph.push((start, i));
    }
  }

  let mut st = HashSet::new();
  let mut visited = vec![false; n];
  let mut que = VecDeque::new();
  que.push_back(0_usize);

  while let Some(top) = que.pop_front() {
    if visited[top] {
        continue;
    }
    st.insert(top);
    visited[top] = true;

    for next in edges[top].iter() {
        if !visited[*next] {
            que.push_back(*next);
        }
    }
  }
  st.remove(&0_usize);

  let sorted = topological_sort(&graph).unwrap();

  println!("{}", sorted.iter().filter(|book| st.contains(*book)).map(|book| book + 1).join(" "));
}

#[derive(Debug, Eq, PartialEq)]
pub enum TopoligicalSortError {
    CycleDetected,
}

type TopologicalSortResult<Node> = Result<Vec<Node>, TopoligicalSortError>;

/// Given a directed graph, modeled as a list of edges from source to destination
/// Uses Kahn's algorithm to either:
///     return the topological sort of the graph
///     or detect if there's any cycle
pub fn topological_sort<Node: Hash + Eq + Copy>(
    edges: &Vec<(Node, Node)>,
) -> TopologicalSortResult<Node> {
    // Preparation:
    //  Build a map of edges, organised from source to destinations
    //  Also, count the number of incoming edges by node
    let mut edges_by_source: HashMap<Node, Vec<Node>> = HashMap::default();
    let mut incoming_edges_count: HashMap<Node, usize> = HashMap::default();
    for (source, destination) in edges {
        incoming_edges_count.entry(*source).or_insert(0); // if we haven't seen this node yet, mark it as having 0 incoming nodes
        edges_by_source // add destination to the list of outgoing edges from source
            .entry(*source)
            .or_insert_with(Vec::default)
            .push(*destination);
        // then make destination have one more incoming edge
        *incoming_edges_count.entry(*destination).or_insert(0) += 1;
    }

    // Now Kahn's algorithm:
    // Add nodes that have no incoming edges to a queue
    let mut no_incoming_edges_q = VecDeque::default();
    for (node, count) in &incoming_edges_count {
        if *count == 0 {
            no_incoming_edges_q.push_back(*node);
        }
    }
    // For each node in this "O-incoming-edge-queue"
    let mut sorted = Vec::default();
    while let Some(no_incoming_edges) = no_incoming_edges_q.pop_back() {
        sorted.push(no_incoming_edges); // since the node has no dependency, it can be safely pushed to the sorted result
        incoming_edges_count.remove(&no_incoming_edges);
        // For each node having this one as dependency
        for neighbour in edges_by_source.get(&no_incoming_edges).unwrap_or(&vec![]) {
            if let Some(count) = incoming_edges_count.get_mut(neighbour) {
                *count -= 1; // decrement the count of incoming edges for the dependent node
                if *count == 0 {
                    // `node` was the last node `neighbour` was dependent on
                    incoming_edges_count.remove(neighbour); // let's remove it from the map, so that we can know if we covered the whole graph
                    no_incoming_edges_q.push_front(*neighbour); // it has no incoming edges anymore => push it to the queue
                }
            }
        }
    }
    if incoming_edges_count.is_empty() {
        // we have visited every node
        Ok(sorted)
    } else {
        // some nodes haven't been visited, meaning there's a cycle in the graph
        Err(TopoligicalSortError::CycleDetected)
    }
}
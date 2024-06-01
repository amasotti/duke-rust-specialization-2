use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{DfsPostOrder, EdgeRef};
use petgraph::Directed;
use std::collections::HashSet;
use petgraph::dot::Dot;

pub fn challenge_5() {
    println!("Challenge 5 - Kosaraju's Algorithm");
    let graph = make_graph();

    println!("Graph: {:?}", Dot::new(&graph));

    let components = kosaraju_scc(&graph);

    for (i, component) in components.iter().enumerate() {
        println!("Component {}: {:?}", i, component);
    }
}

/// Create a graph for the challenge
///
/// The graph is a directed graph with the following nodes and edges (dot notation)
///
/// ```dot
/// digraph {
//     0 [ label = "1" ]
//     1 [ label = "2" ]
//     2 [ label = "3" ]
//     3 [ label = "4" ]
//     4 [ label = "5" ]
//     5 [ label = "6" ]
//     6 [ label = "7" ]
//     0 -> 1 [ label = "1-2" ]
//     1 -> 2 [ label = "2-3" ]
//     2 -> 0 [ label = "3-1" ]
//     1 -> 3 [ label = "2-4" ]
//     3 -> 3 [ label = "4-4" ]
//     4 -> 3 [ label = "4-5" ]
//     4 -> 5 [ label = "5-6" ]
//     5 -> 6 [ label = "6-4" ]
//     6 -> 4 [ label = "7-5" ]
// }
/// ```
fn make_graph() -> Graph<usize, &'static str, Directed> {
    let mut graph = Graph::<usize, &str, Directed>::new();
    let a = graph.add_node(1);
    let b = graph.add_node(2);
    let c = graph.add_node(3);
    let d = graph.add_node(4);
    let e = graph.add_node(5);
    let f = graph.add_node(6);
    let g = graph.add_node(7);

    graph.add_edge(a, b, "1-2");
    graph.add_edge(b, c, "2-3");
    graph.add_edge(c, a, "3-1");

    graph.add_edge(b,d, "2-4");
    graph.add_edge(d,d, "4-4");

    graph.add_edge(e, d, "4-5");
    graph.add_edge(e, f, "5-6");
    graph.add_edge(f, g, "6-4");
    graph.add_edge(g, e, "7-5");


    graph
}

fn kosaraju_scc(graph: &Graph<usize, &str, Directed>) -> Vec<Vec<usize>> {
    let mut stack = Vec::new();
    let mut discovered = HashSet::new();

    // First pass: record the finish times of each node in a stack
    for node in graph.node_indices() {
        if !discovered.contains(&node) {
            dfs_first_walk(node, graph, &mut discovered, &mut stack);
        }
    }

    // Transpose the graph
    let transposed_graph = transpose_graph(graph);

    // clear "discovered nodes", we need to run again DFS
    discovered.clear();

    let mut components = Vec::new();

    // Second pass: collect strongly connected components
    while let Some(node) = stack.pop() {
        if !discovered.contains(&NodeIndex::new(node)) {
            let mut component = Vec::new();
            dfs_collect(NodeIndex::new(node), &transposed_graph, &mut discovered, &mut component);
            components.push(component);
        }
    }

    components
}

/// Transpose the graph
///
/// # Arguments
/// * `graph` - The graph to transpose
///
/// # Returns
///
/// The transposed graph. The nodes are the same,
/// but pay attention to the direction of the edges, they are reversed.
///
fn transpose_graph<'a>(graph: &Graph<usize, &'a str>) -> Graph<usize, &'a str> {
    let mut transposed_graph = Graph::<usize, &str, Directed>::new();
    for node in graph.node_indices() {
        transposed_graph.add_node(node.index());
    }
    for edge in graph.edge_references() {
        transposed_graph.add_edge(edge.target(), edge.source(), *edge.weight());
    }
    transposed_graph
}

/// Perform a depth-first search on the graph and record the finish times of each node in a stack
///
/// # Arguments
/// * `node` - The starting node for the depth-first search
/// * `graph` - The graph to search
/// * `visited` - A set of visited nodes
/// * `stack` - A stack to record the finish times of each node
///
/// # Returns
///
/// The finish times of each node are recorded in the `finish_time` stack
///
/// # How it works
///
/// The DFS iterates over the nodes and if a node has not been visited, it is added to the visited set
/// and its finish time is recorded in the stack
fn dfs_first_walk(
    node: NodeIndex,
    graph: &Graph<usize, &str, Directed>,
    visited: &mut HashSet<NodeIndex>,
    stack: &mut Vec<usize>,
) {
    let mut dfs = DfsPostOrder::new(graph, node);
    while let Some(node) = dfs.next(graph) {
        if visited.insert(node) {
            stack.push(node.index());
        }
    }
}

fn dfs_collect(
    node: NodeIndex,
    graph: &Graph<usize, &str, Directed>,
    visited: &mut HashSet<NodeIndex>,
    component: &mut Vec<usize>,
) {
    let mut dfs = DfsPostOrder::new(graph, node);
    while let Some(node) = dfs.next(graph) {
        if visited.insert(node) {
            component.push(node.index());
            dfs_collect(node, graph, visited, component);
        }
    }
}

fn main() {
    challenge_5();
}

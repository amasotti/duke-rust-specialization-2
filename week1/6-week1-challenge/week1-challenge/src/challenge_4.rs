use petgraph::algo::connected_components;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;

pub fn challenge_4() {
    println!("Challenge 4: Check if graph is fully connected");
    let g = make_graph();
    println!("Graph {:?}", Dot::new(&g));

    if is_fully_connected(&g) {
        println!("Graph is fully connected");
    } else {
        println!("Graph is not fully connected");
    }
}


fn make_graph() -> DiGraph<&'static str, &'static str>{
    let mut graph = DiGraph::new();
    let a = graph.add_node("Node A");
    let b = graph.add_node("Node B");
    let c = graph.add_node("Node C");
    graph.add_edge(a, b, "Rel 1");
    graph.add_edge(b, a, "Rel 2");
    graph.add_edge(c, c, "Rel 3");

    graph
}

/// Check if the graph is fully connected
///
/// The [connected_components](https://docs.rs/petgraph/latest/petgraph/algo/fn.connected_components.html) function
/// from the petgraph crate is used to determine the number of connected components in the graph.
/// If the number of connected components is 1, then the graph is fully connected.
///
/// # Arguments
/// * `graph` - The graph to check
///
/// # Returns
/// * `true` if the graph is fully connected, `false` otherwise
fn is_fully_connected(graph: &DiGraph<&'static str, &'static str>) -> bool {
    connected_components(graph) == 1
}
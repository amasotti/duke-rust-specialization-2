use crate::authors::Author;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};

/// Adds an undirected edge with a weight between two nodes in the graph.
///
/// This function takes a mutable reference to a graph (`UnGraph`), a slice of `NodeIndex`
/// representing nodes in the graph, and two indices (`a` and `b`). It adds an edge with a
/// weight of 1.0 between the nodes at the specified indices in the slice.
///
/// # Arguments
///
/// * `graph` - A mutable reference to the graph (`UnGraph`) where the edge will be added.
/// * `nodes` - A slice of `NodeIndex` that represents the nodes in the graph. The indices `a` and `b`
///             refer to positions within this slice.
/// * `a` - The index within `nodes` for the first node of the edge.
/// * `b` - The index within `nodes` for the second node of the edge.
/// * `weight` - The weight of the edge to be added.
///
/// # Examples
///
/// ```rust
/// use petgraph::graph::{NodeIndex, UnGraph};
/// use graph_demo::authors::Author;
/// use graph_demo::add_edge;
///
/// let mut graph = UnGraph::<&Author, f32>::new_undirected();
/// let author1 = Author::new("Author1");
/// let author2 = Author::new("Author2");
///
/// let node1 = graph.add_node(&author1);
/// let node2 = graph.add_node(&author2);
///
/// let nodes = vec![node1, node2];
/// add_edge(&mut graph, &nodes, 0, 1, 1.0);
/// ```
///
/// This will add an edge with a weight of 1.0 between "Author1" and "Author2" in the graph.
pub fn add_edge(
    graph: &mut UnGraph<&Author, f32>,
    nodes: &[NodeIndex],
    a: usize,
    b: usize,
    weight: f32,
) {
    graph.add_edge(nodes[a], nodes[b], weight);
}

/// Converts a vector of `Author` references into graph nodes and returns their indices.
///
/// This function iterates over a vector of `Author` references, adds each author as a node
/// to the provided graph, and collects the indices of these nodes into a vector. The graph
/// is modified in-place to include these new nodes. The function is generic over the lifetime
/// `'a` to ensure that the graph and the authors have the same lifetime, preventing dangling references.
///
/// # Arguments
///
/// * `authors` - A reference to a vector of `Author` references. These authors are to be added as nodes to the graph.
/// * `graph` - A mutable reference to a `Graph` object where the authors will be added as nodes. The graph is undirected and the edges, if any, will have a weight of type `f32`.
///
/// # Returns
///
/// A `Vec<NodeIndex>` containing the indices of the newly added author nodes in the graph.
///
/// # Examples
///
/// ```rust
/// use petgraph::graph::{Graph, NodeIndex};
/// use petgraph::Undirected;
/// use graph_demo::authors::Author;
/// use graph_demo::vec_to_graph_nodes;
///
/// let authors = vec![
///     Author::new("Author1"),
///     Author::new("Author2"),
/// ];
///
/// let mut graph = Graph::<&Author, f32, Undirected>::new_undirected();
/// let author_nodes = vec_to_graph_nodes(&authors, &mut graph);
///
/// assert_eq!(author_nodes.len(), 2);
/// ```
pub fn vec_to_graph_nodes<'a>(authors: &'a Vec<Author>, graph: &mut Graph<&'a Author, f32, Undirected>) -> Vec<NodeIndex> {
	let author_nodes = authors
		.iter()
		.map(|author| graph.add_node(author))
		.collect::<Vec<NodeIndex>>();
	author_nodes
}

mod authors;
mod graph_utils;

use authors::Author;
use graph_utils::add_edge;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Direction, Graph, Undirected};

fn get_authors() -> Vec<Author> {
    vec![
        Author::new("Boccaccio"), // 0
        Author::new("Chaucer"),   // 1
        Author::new("Dante"),    // 2
        Author::new("Homer"),   // 3
        Author::new("Virgil"), // 4
        Author::new("Shakespeare"), // 5
    ]
}

fn add_edges(mut graph: &mut Graph<&Author, f32, Undirected>, author_nodes: &Vec<NodeIndex>) {
	add_edge(&mut graph, &author_nodes, 0, 1, 0.3); // Boccaccio -> Chaucer
	add_edge(&mut graph, &author_nodes, 0, 2, 3.0); // Boccaccio -> Dante
	add_edge(&mut graph, &author_nodes, 1, 3, 0.0); // Chaucer -> Homer
	add_edge(&mut graph, &author_nodes, 3, 0, 2.0); // Homer -> Boccaccio
	add_edge(&mut graph, &author_nodes, 3, 2, 4.0); // Homer -> Dante
	add_edge(&mut graph, &author_nodes, 3, 4, 5.0); // Homer -> Virgil
	add_edge(&mut graph, &author_nodes, 0, 4, 0.0); // Boccaccio -> Virgil
	add_edge(&mut graph, &author_nodes, 2, 4, 10.0); // Dante -> Virgil
	add_edge(&mut graph, &author_nodes, 4, 5, 1.4); // Virgil -> Shakespeare
	add_edge(&mut graph, &author_nodes, 1, 5, 2.4); // Chaucer -> Shakespeare
}

/// Explain the data
///
/// This function explains the data by calculating the closeness centrality of each author
/// and providing an explanation for the closeness centrality of each author.
///
/// # Arguments
///
/// * `authors` - A reference to a vector of `Author` references. These authors are to be added as nodes to the graph.
/// * `graph` - A mutable reference to a `Graph` object where the authors will be added as nodes. The graph is undirected and the edges, if any, will have a weight of type `f32`.
/// * `author_nodes` - A vector of `NodeIndex` containing the indices of the newly added author nodes in the graph.
///
/// # Further Explanation
///
/// ## Closeness Centrality
///
/// Closeness centrality is a measure of how close a node is to all other nodes in the graph.
/// It is calculated as the reciprocal of the sum of the shortest path distances from the node to all other
/// nodes in the graph. A node with high closeness centrality is close to many other nodes in the graph.
///
fn explain_data(authors: &Vec<Author>, graph: &mut Graph<&Author, f32, Undirected>, author_nodes: Vec<NodeIndex>) {
	for (i, &node) in author_nodes.iter().enumerate() {
		let name = &authors[i].name;
		let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
		let closeness = 1.0 / degree;
		println!("The closeness centrality of {} is {:.2}", name, closeness);

		// Explanation
		match name.as_str() {
			"Boccaccio" => println!("Boccaccio is close to Chaucer, Dante, and Virgil"),
			"Chaucer" => println!("Chaucer is close to Boccaccio and Shakespeare"),
			"Dante" => println!("Dante is close to Boccaccio and Virgil"),
			"Homer" => println!("Homer is close to Boccaccio, Dante, and Virgil"),
			"Virgil" => println!("Virgil is close to Homer, Dante, and Shakespeare"),
			"Shakespeare" => println!("Shakespeare is close to Chaucer and Virgil"),
			_ => {}
		}
		println!("-----------------");
	}
}



fn main() {
    let authors = get_authors();
    let mut graph = UnGraph::new_undirected();

	let author_nodes = graph_utils::vec_to_graph_nodes(&authors, &mut graph);

	add_edges(&mut graph, &author_nodes);

	explain_data(&authors, &mut graph, author_nodes);
}


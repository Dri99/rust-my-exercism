use dot_dsl::graph::Graph;
use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;

fn main() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);
    println!("{}", graph);
}
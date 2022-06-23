pub mod graph {
    use graph_items::edge::*;
    use graph_items::node::*;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::fmt;

    pub mod graph_items;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String,String>
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph::default();
        }

        pub fn with_nodes(self , nodes :& Vec<Node>) -> Self {
            //let mut newNodes : Vec<Node> = ;

            return Graph{nodes: nodes.to_vec(), ..self }
        }

        pub fn with_edges(self , edges :& Vec<Edge>) -> Self {
            return Graph{edges: edges.to_vec(), ..self }
        }
        pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self {
            let hashmap: HashMap<String,String> = attrs.iter()
                .map(|(key, val)| {(key.to_string(), val.to_string())})
                .collect();
            return Graph{attrs: hashmap, ..self };
        }
        pub fn get_node(&self, node: &str) -> Option<&Node> {
            let found = self.nodes.iter()
                .find(|n| {
                    n.name.as_str() == node
                });
            return found;

            //return Some(Node::new(node));
        }
    }

    impl Display for Graph {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            writeln!(f, "graph {{").expect("[Graph::Display] Error");
            for (key, value) in &self.attrs{
                writeln!(f, "\t{}=\"{}\"",key,value).expect("[Graph::Display] Error");
            }
            for node in &self.nodes {
                writeln!(f, "\t{}", node).expect("[Graph::Display] Error");
            }
            for edge in &self.edges {
                writeln!(f, "\t{}", edge).expect("[Graph::Display] Error");
            }
            writeln!(f, "}}").expect("[Graph::Display] Error");
            return Ok(());
        }
    }
}

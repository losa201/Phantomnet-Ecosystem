use serde::{Serialize, Deserialize};
use uuid::Uuid;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
}

pub fn create_graph() -> Graph<Node, ()> {
    let mut graph = Graph::new();
    let a = graph.add_node(Node { id: Uuid::new_v4(), name: "A".into() });
    let b = graph.add_node(Node { id: Uuid::new_v4(), name: "B".into() });
    graph.add_edge(a, b, ());
    graph
}

use std::collections::HashMap;
use std::io::{self, Write};

use crate::node::*;

pub enum GraphKind {
    Digraph,
    Subgraph,
}

pub type AdjList<'a> = HashMap<&'a String, Vec<&'a String>>;

pub struct Graph {
    // Identifier for the graph
    pub name: String,
    pub kind: GraphKind,

    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone)]
pub struct GraphvizSettings {
    pub graph_attrs: Option<String>,
    pub node_attrs: Option<String>,
    pub edge_attrs: Option<String>,
    pub graph_label: Option<String>,
    // pub dark_mode: bool,
}

impl Default for GraphvizSettings {
    fn default() -> GraphvizSettings {
        GraphvizSettings {
            graph_attrs: None,
            node_attrs: None,
            edge_attrs: None,
            graph_label: None,
        }
    }
}

impl Graph {
    pub fn new(name: String, kind: GraphKind, nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        Graph {
            name,
            kind,
            nodes,
            edges,
        }
    }
    pub fn adj_list(&self) -> AdjList<'_> {
        let mut m = HashMap::new();
        for node in &self.nodes {
            m.insert(&node.label, Vec::new());
        }
        for edge in &self.edges {
            m.entry(&edge.from).or_insert_with(Vec::new).push(&edge.to);
        }
        m
    }

    pub fn rev_adj_list(&self) -> AdjList<'_> {
        let mut m = HashMap::new();
        for node in &self.nodes {
            m.insert(&node.label, Vec::new());
        }
        for edge in &self.edges {
            m.entry(&edge.to).or_insert_with(Vec::new).push(&edge.from);
        }
        m
    }

    pub fn get_node_by_label(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label == *label)
    }

    pub fn to_dot<W: Write>(&self, w: &mut W, settings: &GraphvizSettings) -> io::Result<()> {
        match self.kind {
            GraphKind::Digraph => write!(w, "digraph {}", self.name),
            // cluster_ draws a border around the graph
            GraphKind::Subgraph => write!(w, "subgraph cluster_{}", self.name),
        }?;

        if let Some(graph_attrs) = &settings.graph_attrs {
            writeln!(w, r#"    graph [{}];"#, graph_attrs)?;
        }
        if let Some(node_attrs) = &settings.node_attrs {
            writeln!(w, r#"    node [{}];"#, node_attrs)?;
        }
        if let Some(edge_attrs) = &settings.edge_attrs {
            writeln!(w, r#"    edge [{}];"#, edge_attrs)?;
        }

        writeln!(w, " {{")?;

        for node in self.nodes.iter() {
            write!(w, r#"    {} [shape="none", label=<"#, node.label)?;
            node.to_dot(w)?;
            writeln!(w, ">];")?;
        }

        for edge in self.edges.iter() {
            edge.to_dot(w)?;
        }

        writeln!(w, "}}")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    fn get_test_graph() -> Graph {
        let stmts: Vec<String> = vec!["hi".into(), "hell".into()];
        let label1: String = "bb0__0_3".into();
        let style: NodeStyle = Default::default();
        let node1 = Node::new(stmts, label1.clone(), "0".into(), style.clone());

        let stmts: Vec<String> = vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()];
        let label2: String = "bb0__1_3".into();
        let node2 = Node::new(stmts, label2.clone(), "1".into(), style.clone());

        Graph::new(
            "Mir_0_3".into(),
            GraphKind::Digraph,
            vec![node1, node2],
            vec![Edge::new(label1, label2, "return".into())],
        )
    }
    #[test]
    fn test_adj_list() {
        let g = get_test_graph();
        let adj_list = g.adj_list();
    }
}

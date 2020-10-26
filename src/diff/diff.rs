use crate::diff::{match_graphs, DiffGraph};
use crate::graph::Graph;
use std::collections::HashSet;

pub struct DiffResult<'a> {
    pub added: Vec<&'a str>,
    pub removed: Vec<&'a str>,
    pub partial_match: Vec<&'a str>,
}

pub fn diff_graph<'a>(g1: &'a Graph, g2: &'a Graph) -> DiffResult<'a> {
    let d1 = DiffGraph::new(g1);
    let d2 = DiffGraph::new(g2);
    let mapping = match_graphs(&d1, &d2);

    let matched1: HashSet<&str> = mapping.iter().map(|(k, _)| *k).collect();
    let matched2: HashSet<&str> = mapping.iter().map(|(_, v)| *v).collect();

    DiffResult {
        added: g2
            .nodes
            .iter()
            .map(|n| n.label.as_str())
            .filter(|n| !matched2.contains(n))
            .collect(),
        removed: g1
            .nodes
            .iter()
            .map(|n| n.label.as_str())
            .filter(|n| !matched1.contains(n))
            .collect(),
        partial_match: vec![],
    }
}

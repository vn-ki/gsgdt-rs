use crate::diff::{match_graphs, DiffGraph, Match};
use crate::{AdjList, Edge, EdgeStyle, Graph, MultiGraph, NodeStyle};
use std::collections::HashMap;

/// Returns a MultiGraph containing the diff of the two graphs.
/// To be visualized with dot.
pub fn visualize_diff(d1: &DiffGraph, d2: &DiffGraph) -> MultiGraph {
    let matches = match_graphs(d1, d2);

    // TODO: Maybe return the matches as 2 hashmap?
    // no point in doing the conversion twice

    let mut matched1 = HashMap::new();
    let mut matched2 = HashMap::new();
    let mut partial1 = HashMap::new();
    let mut partial2 = HashMap::new();

    for mch in matches {
        match mch {
            Match::Full(m) => {
                matched1.insert(m.from, m.to);
                matched2.insert(m.to, m.from);
            }
            Match::Partial(m) => {
                partial1.insert(m.from, m.to);
                partial2.insert(m.to, m.from);
            }
        }
    }

    let added_style = NodeStyle {
        title_bg: Some("green3".into()),
        ..Default::default()
    };
    let removed_style = NodeStyle {
        title_bg: Some("red2".into()),
        ..Default::default()
    };
    let changed_style = NodeStyle {
        title_bg: Some("yellow".into()),
        ..Default::default()
    };
    let default_style = d1.graph.nodes[0].style.clone();

    let mut edges1: Vec<Edge> = d1.graph.edges.clone();
    let mut edges2: Vec<Edge> = d2.graph.edges.clone();
    let adj_list1 = d1.graph.adj_list();
    let adj_list2 = d2.graph.adj_list();
    let removed_edge_style = EdgeStyle {
        color: Some("red2".to_owned()),
    };
    let added_edge_style = EdgeStyle {
        color: Some("green3".to_owned()),
    };
    colors_edges(
        &mut edges1,
        &matched1,
        &partial1,
        &adj_list2,
        removed_edge_style,
        "_diff1",
    );
    colors_edges(
        &mut edges2,
        &matched2,
        &partial2,
        &adj_list1,
        added_edge_style,
        "_diff2",
    );

    let mut nodes1 = Vec::new();
    for node in &d1.graph.nodes {
        let label = node.label.as_str();
        let mut node_cloned = node.clone();
        node_cloned.label = format!("{}_diff1", node.label);
        if matched1.contains_key(label) {
            node_cloned.style = default_style.clone();
            nodes1.push(node_cloned);
        } else if partial1.contains_key(label) {
            node_cloned.style = changed_style.clone();
            nodes1.push(node_cloned);
        } else {
            node_cloned.style = removed_style.clone();
            nodes1.push(node_cloned);
        }
    }

    let mut nodes2 = Vec::new();
    for node in &d2.graph.nodes {
        let label = node.label.as_str();
        let mut node_cloned = node.clone();
        node_cloned.label = format!("{}_diff2", node.label);
        if matched2.contains_key(label) {
            node_cloned.style = default_style.clone();
            nodes2.push(node_cloned);
        } else if partial2.contains_key(label) {
            node_cloned.style = changed_style.clone();
            nodes2.push(node_cloned);
        } else {
            node_cloned.style = added_style.clone();
            nodes2.push(node_cloned);
        }
    }
    let newg1 = Graph::new("diff1".to_owned(), nodes1, edges1);
    let newg2 = Graph::new("diff2".to_owned(), nodes2, edges2);

    MultiGraph::new("diff".to_owned(), vec![newg1, newg2])
}

fn colors_edges(
    edges: &mut Vec<Edge>,
    matches: &HashMap<&str, &str>,
    partial: &HashMap<&str, &str>,
    adj_list: &AdjList<'_>,
    style: EdgeStyle,
    name_prefix: &str,
) {
    for e in edges.iter_mut() {
        let (from, to) = (e.from.as_str(), e.to.as_str());
        if let Some(matched_lab_from) = matches.get(from).or(partial.get(from)) {
            if let Some(matched_lab_to) = matches.get(to).or(partial.get(to)) {
                // both nodes of the edge are matched
                // TODO: Weird lifetime error
                let children = &adj_list[matched_lab_from.to_string().as_str()];
                if children.iter().any(|s| s == matched_lab_to) {
                    // edge was matched, no need to color
                    e.from.push_str(name_prefix);
                    e.to.push_str(name_prefix);
                    continue;
                }
                // there doesn't exist an edge
                // color this edge
            }
            // the node did not get matched
            // color this edge
        }
        e.style = style.clone();
        // color the edge added
        // TODO: push
        e.from.push_str(name_prefix);
        e.to.push_str(name_prefix);
    }
}

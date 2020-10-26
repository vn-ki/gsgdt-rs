use crate::diff::*;
use crate::levenshtein::distance;
use std::collections::BTreeMap;

pub type Mapping<'a> = BTreeMap<&'a str, &'a str>;

// pub struct Matching {
//     from: String,
//     to: String,
// }
//
// pub enum Match {
//     Full(Matching),
//     Partial(Matching),
// }

// Matches both graphs and returns the mapping of nodes from g1 to g2
pub fn match_graphs<'a>(d1: &'a DiffGraph<'_>, d2: &'a DiffGraph<'_>) -> Mapping<'a> {
    let mapping: BTreeMap<&str, &str> = get_initial_mapping(d1, d2);

    // let _matches: Vec<Match> = mapping
    //     .iter()
    //     .map(|(from, to)| {
    //         Match::Full(Matching {
    //             from: from.to_string(),
    //             to: to.to_string(),
    //         })
    //     })
    //     .collect();

    // we use rev adjacency list because we are going to compare the parents
    // let rev_adj_list1 = d1.graph.rev_adj_list();
    // let rev_adj_list2 = d2.graph.rev_adj_list();
    //
    // let matched_labels: HashSet<&str> = mapping.iter().map(|(_, v)| *v).collect();
    //
    // //merge
    // let prev_mapping = mapping.clone();
    // loop {
    //     let new_mapping: Mapping = HashMap::new();
    //     for (k, v) in prev_mapping.iter() {
    //         let parents_in_1 = rev_adj_list1.get(&k.to_string()).unwrap();
    //         let parents_in_2 = rev_adj_list2.get(&v.to_string()).unwrap();
    //
    //         for n in parents_in_1.iter() {
    //             if let Some(lab) = select(d1, d2, n, parents_in_2)) {
    //                 if !matched_labels.contains(lab) {
    //                     matched_labels.insert(lab);
    //                     mapping.insert(n, lab);
    //                 }
    //             }
    //         }
    //     }
    // }

    mapping
}

fn get_initial_mapping<'a>(d1: &'a DiffGraph<'_>, d2: &'a DiffGraph<'_>) -> Mapping<'a> {
    let mut swapped = false;
    let (d1, d2) = if d1.graph.nodes.len() > d2.graph.nodes.len() {
        swapped = true;
        (d2, d1)
    } else {
        (d1, d2)
    };
    let mut mapping: BTreeMap<&str, &str> = BTreeMap::new();
    let g2_labels: Vec<&str> = d2.graph.nodes.iter().map(|n| n.label.as_str()).collect();

    // TODO: this can be functional
    for node in d1.graph.nodes.iter() {
        if let Some(matched_lab) = select(d1, d2, &node.label, &g2_labels) {
            mapping.insert(&node.label, matched_lab);
        }
    }

    if swapped {
        let mut swapped_map = BTreeMap::new();
        for (k, v) in mapping.into_iter() {
            swapped_map.insert(v, k);
        }
        swapped_map
    } else {
        mapping
    }
}

fn dist_bw_nodes(d1: &DiffGraph<'_>, d2: &DiffGraph<'_>, n1: &str, n2: &str) -> Option<usize> {
    let node1 = d1.graph.get_node_by_label(n1).unwrap();
    let node2 = d2.graph.get_node_by_label(n2).unwrap();

    let tup1 = (
        d1.dist_start[n1] as i64,
        d1.dist_end[n1] as i64,
        node1.stmts.len() as i64,
    );
    let tup2 = (
        d2.dist_start[n2] as i64,
        d2.dist_end[n2] as i64,
        node2.stmts.len() as i64,
    );

    let s1 = node1.stmts.join("");
    let s2 = node2.stmts.join("");
    let dist = distance(&s1, &s2);

    Some(
        ((tup1.0 - tup2.0).pow(2) + (tup1.1 - tup2.1).pow(2) + (tup1.2 - tup2.2).pow(2)) as usize
            + dist,
    )
}

// Selects the most suitable match for n from L
fn select<'a>(
    d1: &'a DiffGraph<'_>,
    d2: &'a DiffGraph<'_>,
    n: &'a str,
    list_of_labs: &[&'a str],
) -> Option<&'a str> {
    let node = d1.graph.get_node_by_label(n).unwrap();
    let node_stmt_len = node.stmts.len();
    let node_stmts = node.stmts.join("");
    list_of_labs.iter()
        .filter(|lab| {
            let other_node = d2.graph.get_node_by_label(lab).unwrap();
            // filter out nodes that may differ by more than 2 lines
            (other_node.stmts.len() as i64 - node.stmts.len() as i64).abs() <= 2
        })
        // TODO: make this filter configurable
        .filter(|lab| {
            let other_node_stmts = d2.graph.get_node_by_label(lab).unwrap().stmts.join("");
            // allow bigger basic blocks have more edits
            // 2 here is arbitary
            distance(&node_stmts, &other_node_stmts) < 2 * node_stmt_len
        })
        .min_by_key(|x| dist_bw_nodes(d1, d2, n, x))
        .map(|x| *x)
}

// fn merge<'a>(mut mapping: Mapping<'a>, new_mapping: Mapping<'a>) -> (Mapping<'a>, bool) {
//     let mut changed = false;
//     for (k, v) in new_mapping.iter() {
//         if !mapping.contains_key(k) {
//             mapping.insert(k, v);
//             changed = true;
//         }
//     }
//
//     (mapping, changed)
// }

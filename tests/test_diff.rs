use gsgdt;
use serde_json;
use std::collections::BTreeMap;

use std::fs::File;
use std::io::prelude::*;

use gsgdt::*;

// #[test]
// fn test_diff() {
//     let style: NodeStyle = NodeStyle {
//         last_stmt_sep: true,
//         ..Default::default()
//     };
//     let settings: GraphvizSettings = Default::default();
//
//     let g = Graph::new(
//         "Mir_0_3".into(),
//         GraphKind::Digraph,
//         vec![
//             Node::new(
//                 vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
//                 "bb0".into(),
//                 "bb0".into(),
//                 style.clone(),
//             ),
//             Node::new(
//                 vec!["resume".into()],
//                 "bb1".into(),
//                 "bb1".into(),
//                 style.clone(),
//             ),
//             Node::new(
//                 vec![
//                     "StorageLive(_2) StorageLive(_3) _3 = &mut _1".into(),
//                     "_2 = Vec::<i32>::push(move _3, const 1_i32)".into(),
//                 ],
//                 "bb2".into(),
//                 "bb2".into(),
//                 style.clone(),
//             ),
//             Node::new(
//                 vec![
//                     "StorageDead(_3) StorageDead(_2) _0 = const ()".into(),
//                     "drop(_1)".into(),
//                 ],
//                 "bb3".into(),
//                 "bb3".into(),
//                 style.clone(),
//             ),
//             Node::new(
//                 vec!["drop(_1)".into()],
//                 "bb4".into(),
//                 "bb4".into(),
//                 style.clone(),
//             ),
//             Node::new(
//                 vec!["StorageDead(_1)".into(), "return".into()],
//                 "bb5".into(),
//                 "bb5".into(),
//                 style.clone(),
//             ),
//         ],
//         vec![
//             Edge::new("bb0".into(), "bb2".into(), "return".into()),
//             Edge::new("bb2".into(), "bb3".into(), "return".into()),
//             Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
//             Edge::new("bb3".into(), "bb5".into(), "return".into()),
//             Edge::new("bb4".into(), "bb1".into(), "return".into()),
//         ],
//     );
//
//     let g2 = Graph::new(
//         "Mir_0_3".into(),
//         GraphKind::Digraph,
//         vec![Node::new(
//             vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
//             "bb0".into(),
//             "bb0".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["resume".into()],
//             "bb1".into(),
//             "bb1".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["StorageLive(_2) StorageLive(_3) _3 = &mut _1".into(), "_2 = Vec::<i32>::push(move _3, const 1_i32)".into()],
//             "bb2".into(),
//             "bb2".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["StorageDead(_3) StorageDead(_2) StorageLive(_4) StorageLive(_5) _5 = &mut _1".into(), "_4 = Vec::<i32>::push(move _5, const 2_i32)".into()],
//             "bb3".into(),
//             "bb3".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["drop(_1)".into()],
//             "bb4".into(),
//             "bb4".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["StorageDead(_5) StorageDead(_4) _0 = const ()".into(), "drop(_1)".into()],
//             "bb5".into(),
//             "bb5".into(),
//             style.clone(),
//         ),
//         Node::new(
//             vec!["StorageDead(_1)".into(), "return".into()],
//             "bb6".into(),
//             "bb6".into(),
//             style.clone(),
//         )],
//         vec![
//             Edge::new("bb0".into(), "bb2".into(), "return".into()),
//             Edge::new("bb2".into(), "bb3".into(), "return".into()),
//             Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
//             Edge::new("bb3".into(), "bb5".into(), "return".into()),
//             Edge::new("bb3".into(), "bb4".into(), "unwind".into()),
//             Edge::new("bb4".into(), "bb1".into(), "return".into()),
//             Edge::new("bb5".into(), "bb6".into(), "return".into())
//         ],
//         );
//
//     let d1 = DiffGraph::new(&g);
//     let d2 = DiffGraph::new(&g2);
//     let mapping = match_graphs(&d1, &d2);
//     dbg!("{:#?}", mapping);
//
//     let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
//     let mut f2 = std::fs::File::create("test2.dot").expect("create failed");
//     g.to_dot(&mut f1, &settings).expect("can't fail");
//     g2.to_dot(&mut f2, &settings).expect("can't fail");
// }

fn read_graph_from_file(file: &str) -> Graph{
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

#[test]
fn test_diff_2() {
    let g1 = read_graph_from_file("tests/graph1.json");
    let g2 = read_graph_from_file("tests/graph2.json");

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let mapping = match_graphs(&d1, &d2);
    let expected: BTreeMap<_, _> = [
        ("bb0", "bb0"),
        ("bb1", "bb1"),
        ("bb10", "bb10"),
        ("bb11", "bb11"),
        ("bb12", "bb12"),
        ("bb13", "bb13"),
        ("bb14", "bb14"),
        ("bb18", "bb7"),
        ("bb2", "bb2"),
        ("bb26", "bb15"),
        ("bb3", "bb3"),
        ("bb4", "bb4"),
        ("bb5", "bb5"),
        ("bb6", "bb6"),
        ("bb8", "bb8"),
        ("bb9", "bb9"),
    ].iter().cloned().collect();

    // dbg!("{:#?}", mapping);
    assert_eq!(mapping, expected);

    let settings: GraphvizSettings = Default::default();
    let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
    let mut f2 = std::fs::File::create("test2.dot").expect("create failed");
    g1.to_dot(&mut f1, &settings).expect("can't fail");
    g2.to_dot(&mut f2, &settings).expect("can't fail");
}

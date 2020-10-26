#![allow(rustc::default_hash_types)]
mod diff;
mod graph;
mod node;
mod levenshtein;
mod util;

pub use diff::*;
pub use graph::*;
pub use node::*;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let stmts: Vec<String> = vec!["hi".into(), "hell".into()];
        let label1: String = "bb0__0_3".into();
        let style: NodeStyle = Default::default();
        let node1 = Node::new(stmts, label1.clone(), "0".into(), style.clone());

        let stmts: Vec<String> = vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()];
        let label2: String = "bb0__1_3".into();
        let node2 = Node::new(stmts, label2.clone(), "1".into(), style.clone());

        let g = Graph::new(
            "Mir_0_3".into(),
            GraphKind::Digraph,
            vec![node1, node2],
            vec![Edge::new(label1, label2, "return".into())],
        );

        let mut file = std::fs::File::create("test.dot").expect("create failed");

        let settings: GraphvizSettings = Default::default();
        g.to_dot(&mut file, settings).expect("can't fail");

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_diff() {
        let style: NodeStyle = NodeStyle { last_stmt_sep: true, ..Default::default() };
        let settings: GraphvizSettings = Default::default();

        let g = Graph::new(
            "Mir_0_3".into(),
            GraphKind::Digraph,
            vec![
            Node::new(
                vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
                "bb0".into(),
                "bb0".into(),
                style.clone(),
            ),
            Node::new(
                vec!["resume".into()],
                "bb1".into(),
                "bb1".into(),
                style.clone(),
            ),
            Node::new(
                vec![
                "StorageLive(_2) StorageLive(_3) _3 = &mut _1".into(),
                "_2 = Vec::<i32>::push(move _3, const 1_i32)".into(),
                ],
                "bb2".into(),
                "bb2".into(),
                style.clone(),
            ),
            Node::new(
                vec![
                "StorageDead(_3) StorageDead(_2) _0 = const ()".into(),
                "drop(_1)".into(),
                ],
                "bb3".into(),
                "bb3".into(),
                style.clone(),
            ),
            Node::new(
                vec!["drop(_1)".into()],
                "bb4".into(),
                "bb4".into(),
                style.clone(),
            ),
            Node::new(
                vec!["StorageDead(_1)".into(), "return".into()],
                "bb5".into(),
                "bb5".into(),
                style.clone(),
            ),
            ],
            vec![
                Edge::new("bb0".into(), "bb2".into(), "return".into()),
                Edge::new("bb2".into(), "bb3".into(), "return".into()),
                Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
                Edge::new("bb3".into(), "bb5".into(), "return".into()),
                Edge::new("bb4".into(), "bb1".into(), "return".into()),
            ],
            );

        let g2 = Graph::new(
            "Mir_0_3".into(),
            GraphKind::Digraph,
            vec![Node::new(
                vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
                "bb0".into(),
                "bb0".into(),
                style.clone(),
            ),
            Node::new(
                vec!["resume".into()],
                "bb1".into(),
                "bb1".into(),
                style.clone(),
            ),
            Node::new(
                vec!["StorageLive(_2) StorageLive(_3) _3 = &mut _1".into(), "_2 = Vec::<i32>::push(move _3, const 1_i32)".into()],
                "bb2".into(),
                "bb2".into(),
                style.clone(),
            ),
            Node::new(
                vec!["StorageDead(_3) StorageDead(_2) StorageLive(_4) StorageLive(_5) _5 = &mut _1".into(), "_4 = Vec::<i32>::push(move _5, const 2_i32)".into()],
                "bb3".into(),
                "bb3".into(),
                style.clone(),
            ),
            Node::new(
                vec!["drop(_1)".into()],
                "bb4".into(),
                "bb4".into(),
                style.clone(),
            ),
            Node::new(
                vec!["StorageDead(_5) StorageDead(_4) _0 = const ()".into(), "drop(_1)".into()],
                "bb5".into(),
                "bb5".into(),
                style.clone(),
            ),
            Node::new(
                vec!["StorageDead(_1)".into(), "return".into()],
                "bb6".into(),
                "bb6".into(),
                style.clone(),
            )],
            vec![
                Edge::new("bb0".into(), "bb2".into(), "return".into()),
                Edge::new("bb2".into(), "bb3".into(), "return".into()),
                Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
                Edge::new("bb3".into(), "bb5".into(), "return".into()),
                Edge::new("bb3".into(), "bb4".into(), "unwind".into()),
                Edge::new("bb4".into(), "bb1".into(), "return".into()),
                Edge::new("bb5".into(), "bb6".into(), "return".into())
            ],
            );

        let d1 = DiffGraph::new(&g);
        let d2 = DiffGraph::new(&g2);
        let mapping = match_graphs(&d1, &d2);
        dbg!("{:#?}", mapping);

        let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
        let mut f2 = std::fs::File::create("test2.dot").expect("create failed");
        g.to_dot(&mut f1, &settings).expect("can't fail");
        g2.to_dot(&mut f2, &settings).expect("can't fail");
    }
}

# gsgdt

**G**eneric **S**tringly typed **G**raph **D**ata**T**ype

```rust
fn main() {
        let label1: String = "bb0__0_3".into();
        let label2: String = "bb0__1_3".into();
        let style: NodeStyle = Default::default();

        let nodes = vec![
            Node::new(
                vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()],
                label1.clone(),
                "0".into(),
                style.clone(),
            ),
            Node::new(
                vec!["return".into()],
                label2.clone(),
                "1".into(),
                style.clone(),
            ),
        ];

        let g = Graph::new(
            "Mir_0_3".into(),
            nodes,
            vec![Edge::new(label1, label2, "return".into())],
        );
}
```

## Diffing two graphs

`gsgdt` offers functions which can be used find an approximate graph isomorphism
between two graphs. The algorithm matches using the content of the node (statements)
and the relative position of the node (the structure of the graph). This can then be rendered as a diff using graphviz.

![Diff](https://i.imgur.com/jLObxBs.png)

The green nodes and edges are newly added. The red nodes and edges are removed.

The code which produced the above image.
```rust
fn test_diff_readme() {
    let style: NodeStyle = Default::default();
    let g1 = Graph::new(
        "small".into(),
        vec![
            Node::new(
                vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()],
                "bb0".into(),
                "bb0".into(),
                style.clone(),
            ),
            Node::new(
                vec!["return".into()],
                "bb1".into(),
                "bb1".into(),
                style.clone(),
            ),
        ],
        vec![Edge::new("bb0".into(), "bb1".into(), "return".into())],
    );
    let g2 = Graph::new(
        "small".into(),
        vec![
            Node::new(
                vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()],
                "bb0".into(),
                "bb0".into(),
                style.clone(),
            ),
            Node::new(
                vec!["_2 = const 2_i32".into(), "_3 = const 3_i32".into()],
                "bb1".into(),
                "bb1".into(),
                style.clone(),
            ),
            Node::new(
                vec!["return".into()],
                "bb2".into(),
                "bb2".into(),
                style.clone(),
            ),
        ],
        vec![
            Edge::new("bb0".into(), "bb1".into(), "return".into()),
            Edge::new("bb1".into(), "bb2".into(), "return".into()),
        ],
    );

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let settings: GraphvizSettings = Default::default();

    let mg = visualize_diff(&d1, &d2);
    let mut f1 = std::fs::File::create("test1.dot").expect("create failed");

    mg.to_dot(&mut f1, &settings).unwrap();
}
```

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
            GraphKind::Digraph,
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
    let g1_json = r#"
        {
          "name": "small",
          "nodes": [
            {
              "label": "bb0", "style": { "title_bg": null, "last_stmt_sep": false }, "title": "bb0",
              "stmts": [
                "StorageLive(_1)",
                "_1 = Vec::<i32>::new()"
              ]
            },
            {
              "label": "bb1", "style": { "title_bg": null, "last_stmt_sep": false }, "title": "bb1",
              "stmts": [
                "resume"
              ]
            }
          ],
          "edges": [
            { "from": "bb0", "to": "bb1", "style":{"color":null}, "label": "return" }
          ]
        }
    "#;
    let g2_json = r#"
        {
          "name": "small",
          "nodes": [
            {
              "label": "bb0", "style": { "title_bg": null, "last_stmt_sep": false }, "title": "bb0",
              "stmts": [
                "StorageLive(_1)",
                "_1 = Vec::<i32>::new()"
              ]
            },
            {
              "label": "bb1", "style": { "title_bg": null, "last_stmt_sep": false }, "title": "bb0",
              "stmts": [
                "StorageLive(_2)",
                "_2 = Vec::<i32>::new()"
              ]
            },
            {
              "label": "bb2", "style": { "title_bg": null, "last_stmt_sep": false }, "title": "bb1",
              "stmts": [
                "resume"
              ]
            }
          ],
          "edges": [
            { "from": "bb0", "to": "bb1", "style":{"color":null}, "label": "return" },
            { "from": "bb1", "to": "bb2", "style":{"color":null}, "label": "return" }
          ]
        }
    "#;
    let g1: Graph = serde_json::from_str(g1_json).unwrap();
    let g2: Graph = serde_json::from_str(g2_json).unwrap();

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let settings: GraphvizSettings = Default::default();

    let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
    let mg = visualize_diff(&d1, &d2);

    mg.to_dot(&mut f1, &settings).unwrap();
}
```

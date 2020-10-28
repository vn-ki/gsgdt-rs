use gsgdt;
mod helpers;
use helpers::*;

use gsgdt::*;

#[test]
fn test_diff_2() {
    let g1 = read_graph_from_file("tests/graph1.json");
    let g2 = read_graph_from_file("tests/graph2.json");

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let mapping = match_graphs(&d1, &d2);
    let expected = vec![
        Match::Full(Matching::new("bb0", "bb0")),
        Match::Full(Matching::new("bb1", "bb1")),
        Match::Full(Matching::new("bb10", "bb10")),
        Match::Full(Matching::new("bb11", "bb11")),
        Match::Full(Matching::new("bb12", "bb12")),
        Match::Full(Matching::new("bb13", "bb13")),
        Match::Full(Matching::new("bb14", "bb14")),
        Match::Full(Matching::new("bb18", "bb7")),
        Match::Full(Matching::new("bb2", "bb2")),
        Match::Full(Matching::new("bb26", "bb15")),
        Match::Full(Matching::new("bb3", "bb3")),
        Match::Full(Matching::new("bb4", "bb4")),
        Match::Full(Matching::new("bb5", "bb5")),
        Match::Full(Matching::new("bb6", "bb6")),
        Match::Full(Matching::new("bb8", "bb8")),
        Match::Full(Matching::new("bb9", "bb9")),
    ];

    // dbg!("{:#?}", mapping);
    assert_eq!(mapping, expected);

    let settings: GraphvizSettings = Default::default();
    let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
    let mut f2 = std::fs::File::create("test2.dot").expect("create failed");
    g1.to_dot(&mut f1, &settings, false).expect("can't fail");
    g2.to_dot(&mut f2, &settings, false).expect("can't fail");
}

#[test]
fn test_diff_vis() {
    let g1 = read_graph_from_file("tests/graph1.json");
    let g2 = read_graph_from_file("tests/graph2.json");

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let settings: GraphvizSettings = Default::default();

    let mut f1 = std::fs::File::create("test1.dot").expect("create failed");
    let mg = visualize_diff(&d1, &d2);

    mg.to_dot(&mut f1, &settings).unwrap();
}

#[test]
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
            { "from": "bb0",
              "to": "bb1",
              "style":{"color":null},
              "label": "return"
            }
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
            {
                "from": "bb0",
                "to": "bb1",
                "style":{"color":null},
                "label": "return"
            },
            {
                "from": "bb1",
                "to": "bb2",
                "style":{"color":null},
                "label": "return"
            }
          ]
        }
    "#;
    let g1: Graph = serde_json::from_str(g1_json).unwrap();
    let g2: Graph = serde_json::from_str(g2_json).unwrap();

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let settings: GraphvizSettings = Default::default();

    let mg = visualize_diff(&d1, &d2);

    let mut buf = Vec::new();
    mg.to_dot(&mut buf, &settings).unwrap();

    let expected = r#"digraph diff {
subgraph cluster_diff1 {
    bb0_diff1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_1)<br/></td></tr><tr><td align="left">_1 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb1_diff1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left">resume</td></tr></table>>];
    bb0_diff1 -> bb1_diff1 [label="return" color="red"];
}
subgraph cluster_diff2 {
    bb0_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_1)<br/></td></tr><tr><td align="left">_1 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb1_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="green" align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_2)<br/></td></tr><tr><td align="left">_2 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb2_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left">resume</td></tr></table>>];
    bb0_diff2 -> bb1_diff2 [label="return" color="green3"];
    bb1_diff2 -> bb2_diff2 [label="return" color="green3"];
}
}
"#;
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}

#[test]
fn test_diff_partial() {
    let g1_json = r#"
        {
          "name": "small",
        "nodes": [
          {
            "label": "bb0", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb0",
            "stmts": [
              "StorageLive(_1)",
              "_1 = Vec::<i32>::new()"
            ]
          },
          {
            "label": "bb1", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb0",
            "stmts": [
              "StorageLive(_2)"
            ]
          },
          {
            "label": "bb2", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb1",
            "stmts": [
              "resume"
            ]
          }
        ],
        "edges": [
          {
              "from": "bb0",
              "to": "bb1",
              "style":{"color":null},
              "label": "return"
          },
          {
              "from": "bb1",
              "to": "bb2",
              "style":{"color":null},
              "label": "return"
          }
          ]
        }
    "#;
    let g2_json = r#"
        {
          "name": "small",
          "nodes": [
            {
              "label": "bb0", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb0",
              "stmts": [
                "StorageLive(_1)",
                "_1 = Vec::<i32>::new()"
              ]
            },
            {
              "label": "bb1", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb0",
              "stmts": [
                "StorageLive(_2)",
                "_2 = Vec::<i32>::new()"
              ]
            },
            {
              "label": "bb2", "style": { "title_bg": "lightgrey", "last_stmt_sep": false }, "title": "bb1",
              "stmts": [
                "resume"
              ]
            }
          ],
          "edges": [
            {
                "from": "bb0",
                "to": "bb1",
                "style":{"color":null},
                "label": "return"
            },
            {
                "from": "bb1",
                "to": "bb2",
                "style":{"color":null},
                "label": "return"
            }
          ]
        }
    "#;
    let g1: Graph = serde_json::from_str(g1_json).unwrap();
    let g2: Graph = serde_json::from_str(g2_json).unwrap();

    let d1 = DiffGraph::new(&g1);
    let d2 = DiffGraph::new(&g2);
    let settings: GraphvizSettings = Default::default();

    let mut buf = Vec::new();
    let mg = visualize_diff(&d1, &d2);

    mg.to_dot(&mut buf, &settings).unwrap();

    let expected = r#"digraph diff {
subgraph cluster_diff1 {
    bb0_diff1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="lightgrey" align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_1)<br/></td></tr><tr><td align="left">_1 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb1_diff1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="yellow" align="center" colspan="1">bb0</td></tr><tr><td align="left">StorageLive(_2)</td></tr></table>>];
    bb2_diff1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="lightgrey" align="center" colspan="1">bb1</td></tr><tr><td align="left">resume</td></tr></table>>];
    bb0_diff1 -> bb1_diff1 [label="return"];
    bb1_diff1 -> bb2_diff1 [label="return"];
}
subgraph cluster_diff2 {
    bb0_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="lightgrey" align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_1)<br/></td></tr><tr><td align="left">_1 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb1_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="yellow" align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">StorageLive(_2)<br/></td></tr><tr><td align="left">_2 = Vec::&lt;i32&gt;::new()</td></tr></table>>];
    bb2_diff2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="lightgrey" align="center" colspan="1">bb1</td></tr><tr><td align="left">resume</td></tr></table>>];
    bb0_diff2 -> bb1_diff2 [label="return"];
    bb1_diff2 -> bb2_diff2 [label="return"];
}
}
"#;
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}

use gsgdt::*;
mod helpers;
use helpers::*;

#[test]
fn test_multigraph_render() {
    let g1 = get_small_graph();
    let g2 = get_small_graph();
    let settings: GraphvizSettings = Default::default();

    let mg = MultiGraph::new("testgraph".into(), vec![g1, g2]);
    let mut buf = Vec::new();
    let expected = r#"digraph testgraph {
subgraph cluster_small {
    label="small";
    bb0 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">_1 = const 1_i32<br/></td></tr><tr><td align="left">_2 = const 2_i32</td></tr></table>>];
    bb1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left" balign="left">_2 = const 2_i32<br/></td></tr><tr><td align="left">_3 = const 3_i32</td></tr></table>>];
    bb2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb2</td></tr><tr><td align="left">return</td></tr></table>>];
    bb0 -> bb1 [label="return"];
    bb1 -> bb2 [label="return"];
}
subgraph cluster_small {
    label="small";
    bb0 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">_1 = const 1_i32<br/></td></tr><tr><td align="left">_2 = const 2_i32</td></tr></table>>];
    bb1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left" balign="left">_2 = const 2_i32<br/></td></tr><tr><td align="left">_3 = const 3_i32</td></tr></table>>];
    bb2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb2</td></tr><tr><td align="left">return</td></tr></table>>];
    bb0 -> bb1 [label="return"];
    bb1 -> bb2 [label="return"];
}
}
"#;
    mg.to_dot(&mut buf, &settings).unwrap();
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}

#[test]
fn test_multigraph_with_one() {
    let g1 = get_small_graph();
    let settings: GraphvizSettings = Default::default();

    let mg = MultiGraph::new("testgraph".into(), vec![g1]);
    let mut buf = Vec::new();
    let expected = r#"digraph small {
    bb0 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb0</td></tr><tr><td align="left" balign="left">_1 = const 1_i32<br/></td></tr><tr><td align="left">_2 = const 2_i32</td></tr></table>>];
    bb1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb1</td></tr><tr><td align="left" balign="left">_2 = const 2_i32<br/></td></tr><tr><td align="left">_3 = const 3_i32</td></tr></table>>];
    bb2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td  align="center" colspan="1">bb2</td></tr><tr><td align="left">return</td></tr></table>>];
    bb0 -> bb1 [label="return"];
    bb1 -> bb2 [label="return"];
}
"#;
    mg.to_dot(&mut buf, &settings).unwrap();
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}

use gsgdt;
use serde_json;

use std::fs::File;
use std::io::prelude::*;

use gsgdt::*;

pub fn read_graph_from_file(file: &str) -> Graph {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

pub fn get_small_graph() -> Graph {
    let style: NodeStyle = Default::default();

    Graph::new(
        "small".into(),
        vec![
            Node::from_list(
                vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()],
                "bb0".into(),
                "bb0".into(),
                style.clone(),
            ),
            Node::from_list(
                vec!["_2 = const 2_i32".into(), "_3 = const 3_i32".into()],
                "bb1".into(),
                "bb1".into(),
                style.clone(),
            ),
            Node::from_list(
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
    )
}

pub fn get_graph_1() -> Graph {
    let style: NodeStyle = Default::default();

    Graph::new(
        "Mir_0_3".into(),
        vec![Node::from_list(
            vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
            "bb0".into(), "bb0".into(), style.clone()
        ),
        Node::from_list(
            vec!["resume".into()],
            "bb1".into(), "bb1".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_2)".into(), "StorageLive(_3)".into(), "(_3.0: i32) = const 1_i32".into(), "(_3.1: i32) = const 10_i32".into(), "_2 = <std::ops::Range<i32> as IntoIterator>::into_iter(move _3)".into()],
            "bb2".into(), "bb2".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_3)".into(), "StorageLive(_4)".into(), "_4 = move _2".into(), "goto".into()],
            "bb3".into(), "bb3".into(), style.clone()
        ),
        Node::from_list(
            vec!["drop(_1)".into()],
            "bb4".into(), "bb4".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_5)".into(), "StorageLive(_6)".into(), "StorageLive(_7)".into(), "StorageLive(_8)".into(), "_8 = &mut _4 _7 = &mut (*_8)".into(), "_6 = <std::ops::Range<i32> as Iterator>::next(move _7)".into()],
            "bb5".into(), "bb5".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_7)".into(), "_9 = discriminant(_6)".into(), "switchInt(move _9)".into()],
            "bb6".into(), "bb6".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_8)".into(), "StorageDead(_6)".into(), "StorageDead(_5)".into(), "StorageDead(_4)".into(), "StorageDead(_2)".into(), "StorageLive(_21)".into(), "StorageLive(_22)".into(), "(_22.0: i32) = const 1_i32".into(), "(_22.1: i32) = const 10_i32".into(), "_21 = <std::ops::Range<i32> as IntoIterator>::into_iter(move _22)".into()],
            "bb7".into(), "bb7".into(), style.clone()
        ),
        Node::from_list(
            vec!["unreachable".into()],
            "bb8".into(), "bb8".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_10)".into(), "_10 = ((_6 as Some).0: i32) StorageLive(_11)".into(), "_11 = _10 _5 = move _11 StorageDead(_11)".into(), "StorageDead(_10)".into(), "StorageDead(_8)".into(), "StorageDead(_6)".into(), "StorageLive(_12)".into(), "_12 = _5 StorageLive(_13)".into(), "StorageLive(_14)".into(), "_14 = _12 _15 = const false".into(), "_16 = Eq(_14, const i32::MIN) _17 = BitAnd(move _15, move _16)".into(), "assert(!move _17, attempt to compute the remainder of `{} % {}` which would overflow, _14, const 2_i32)".into()],
            "bb9".into(), "bb9".into(), style.clone()
        ),
        Node::from_list(
            vec!["_13 = Rem(move _14, const 2_i32)".into(),"StorageDead(_14)".into(), "switchInt(move _13)".into()],
            "bb10".into(), "bb10".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_13)".into(), "goto".into()],
            "bb11".into(), "bb11".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_13)".into(), "StorageLive(_18)".into(), "StorageLive(_19)".into(), "_19 = &mut _1 StorageLive(_20)".into(), "_20 = _12".into(), "_18 = Vec::<i32>::push(move _19, move _20)".into()],
            "bb12".into(), "bb12".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_20)".into(), "StorageDead(_19)".into(), "StorageDead(_18)".into(), "goto".into()],
            "bb13".into(), "bb13".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_12)".into(), "StorageDead(_5)".into(), "goto".into()],
            "bb14".into(), "bb14".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_22)".into(), "StorageLive(_23)".into(), "_23 = move _21".into(), "goto".into()],
            "bb15".into(), "bb15".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_24)".into(), "StorageLive(_25)".into(), "StorageLive(_26)".into(), "StorageLive(_27)".into(), "_27 = &mut _23 _26 = &mut (*_27)".into(), "_25 = <std::ops::Range<i32> as Iterator>::next(move _26)".into()],
            "bb16".into(), "bb16".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_26)".into(), "_28 = discriminant(_25)".into(), "switchInt(move _28)".into()],
            "bb17".into(), "bb17".into(), style.clone()
        ),
        Node::from_list(
            vec!["_0 = const () StorageDead(_27)".into(), "StorageDead(_25)".into(), "StorageDead(_24)".into(), "StorageDead(_23)".into(), "StorageDead(_21)".into(), "drop(_1)".into()],
            "bb18".into(), "bb18".into(), style.clone()
        ),
        Node::from_list(
            vec!["unreachable".into()],
            "bb19".into(), "bb19".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_29)".into(), "_29 = ((_25 as Some).0: i32)".into(), "StorageLive(_30)".into(), "_30 = _29 _24 = move _30 StorageDead(_30)".into(), "StorageDead(_29)".into(), "StorageDead(_27)".into(), "StorageDead(_25)".into(), "StorageLive(_31)".into(), "_31 = _24 StorageLive(_32)".into(), "StorageLive(_33)".into(), "_33 = _31 _34 = const false".into(), "_35 = Eq(_33, const i32::MIN) _36 = BitAnd(move _34, move _35)".into(), "assert(!move _36, attempt to compute the remainder of `{} % {}` which would overflow, _33, const 3_i32)".into()],
            "bb20".into(), "bb20".into(), style.clone()
        ),
        Node::from_list(
            vec!["_32 = Rem(move _33, const 3_i32)".into(), "StorageDead(_33)".into(), "switchInt(move _32)".into()],
            "bb21".into(), "bb21".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_32)".into(), "goto".into()],
            "bb22".into(), "bb22".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_32)".into(), "StorageLive(_37)".into(), "StorageLive(_38)".into(), "_38 = &mut _1 StorageLive(_39)".into(), "_39 = _31".into(), "_37 = Vec::<i32>::push(move _38, move _39)".into()],
            "bb23".into(), "bb23".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_39)".into(), "StorageDead(_38)".into(), "StorageDead(_37)".into(), "goto".into()],
            "bb24".into(), "bb24".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_31)".into(), "StorageDead(_24)".into(), "goto".into()],
            "bb25".into(), "bb25".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_1)".into(), "return".into()],
            "bb26".into(), "bb26".into(), style.clone()
        )],
        vec![Edge::new("bb0".into(), "bb2".into(), "return".into()),
        Edge::new("bb2".into(), "bb3".into(), "return".into()),
        Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb3".into(), "bb5".into(), "".into()),
        Edge::new("bb4".into(), "bb1".into(), "return".into()),
        Edge::new("bb5".into(), "bb6".into(), "return".into()),
        Edge::new("bb5".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb6".into(), "bb7".into(), "0_isize".into()),
        Edge::new("bb6".into(), "bb9".into(), "1_isize".into()),
        Edge::new("bb6".into(), "bb8".into(), "otherwise".into()),
        Edge::new("bb7".into(), "bb15".into(), "return".into()),
        Edge::new("bb7".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb9".into(), "bb10".into(), "success".into()),
        Edge::new("bb9".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb10".into(), "bb12".into(), "0_i32".into()),
        Edge::new("bb10".into(), "bb11".into(), "otherwise".into()),
        Edge::new("bb11".into(), "bb14".into(), "".into()),
        Edge::new("bb12".into(), "bb13".into(), "return".into()),
        Edge::new("bb12".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb13".into(), "bb14".into(), "".into()),
        Edge::new("bb14".into(), "bb5".into(), "".into()),
        Edge::new("bb15".into(), "bb16".into(), "".into()),
        Edge::new("bb16".into(), "bb17".into(), "return".into()),
        Edge::new("bb16".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb17".into(), "bb18".into(), "0_isize".into()),
        Edge::new("bb17".into(), "bb20".into(), "1_isize".into()),
        Edge::new("bb17".into(), "bb19".into(), "otherwise".into()),
        Edge::new("bb18".into(), "bb26".into(), "return".into()),
        Edge::new("bb20".into(), "bb21".into(), "success".into()),
        Edge::new("bb20".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb21".into(), "bb23".into(), "0_i32".into()),
        Edge::new("bb21".into(), "bb22".into(), "otherwise".into()),
        Edge::new("bb22".into(), "bb25".into(), "".into()),
        Edge::new("bb23".into(), "bb24".into(), "return".into()),
        Edge::new("bb23".into(), "bb4".into(), "unwind".into()),
        Edge::new("bb24".into(), "bb25".into(), "".into()),
        Edge::new("bb25".into(), "bb16".into(), "".into()),
        ],
    )
}

pub fn get_graph_2() -> Graph {
    let style: NodeStyle = Default::default();

    Graph::new(
        "Mir_0_3".into(),
        vec![Node::from_list(
            vec!["StorageLive(_1)".into(), "_1 = Vec::<i32>::new()".into()],
            "bb0".into(), "bb0".into(), style.clone()
        ),
        Node::from_list(
            vec!["resume".into()],
            "bb1".into(), "bb1".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_2)".into(), "StorageLive(_3)".into(), "(_3.0: i32)".into(), "= const 1_i32".into(), "(_3.1: i32)".into(), "= const 10_i32".into(), "_2 = <std::ops::Range<i32> as IntoIterator>::into_iter(move _3)".into()],
            "bb2".into(), "bb2".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_3)".into(), "StorageLive(_4)".into(), "_4 = move _2".into(), "goto".into()],
            "bb3".into(), "bb3".into(), style.clone()
        ),
        Node::from_list(
            vec!["drop(_1)".into()],
            "bb4".into(), "bb4".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_5)".into(), "StorageLive(_6)".into(), "StorageLive(_7)".into(), "StorageLive(_8)".into(), "_8 = &mut _4 _7 = &mut (*_8)".into(), "_6 = <std::ops::Range<i32> as Iterator>::next(move _7)".into()],
            "bb5".into(), "bb5".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_7)".into(), "_9 = discriminant(_6)".into(), "switchInt(move _9)".into()],
            "bb6".into(), "bb6".into(), style.clone()
        ),
        Node::from_list(
            vec!["_0 = const () StorageDead(_8)".into(), "StorageDead(_6)".into(), "StorageDead(_5)".into(), "StorageDead(_4)".into(), "StorageDead(_2)".into(), "drop(_1)".into()],
            "bb7".into(), "bb7".into(), style.clone()
        ),
        Node::from_list(
            vec!["unreachable".into()],
            "bb8".into(), "bb8".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageLive(_10)".into(), "_10 = ((_6 as Some).0: i32)".into(), "StorageLive(_11)".into(), "_11 = _10 _5 = move _11 StorageDead(_11)".into(), "StorageDead(_10)".into(), "StorageDead(_8)".into(), "StorageDead(_6)".into(), "StorageLive(_12)".into(), "_12 = _5 StorageLive(_13)".into(), "StorageLive(_14)".into(), "_14 = _12 _15 = const false".into(), "_16 = Eq(_14, const i32::MIN) _17 = BitAnd(move _15, move _16)".into(), "assert(!move _17, attempt to compute the remainder of `{} % {}` which would overflow, _14, const 3_i32)".into()],
            "bb9".into(), "bb9".into(), style.clone()
        ),
        Node::from_list(
            vec!["_13 = Rem(move _14, const 3_i32)".into(), "StorageDead(_14)".into(), "switchInt(move _13)".into()],
            "bb10".into(), "bb10".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_13)".into(), "goto".into()],
            "bb11".into(), "bb11".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_13)".into(), "StorageLive(_18)".into(), "StorageLive(_19)".into(), "_19 = &mut _1 StorageLive(_20)".into(), "_20 = _12".into(), "_18 = Vec::<i32>::push(move _19, move _20)".into()],
            "bb12".into(), "bb12".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_20)".into(), "StorageDead(_19)".into(), "StorageDead(_18)".into(), "goto".into()],
            "bb13".into(), "bb13".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_12)".into(), "StorageDead(_5)".into(), "goto".into()],
            "bb14".into(), "bb14".into(), style.clone()
        ),
        Node::from_list(
            vec!["StorageDead(_1)".into(), "return".into()],
            "bb15".into(), "bb15".into(), style.clone()
        )],
        vec![
            Edge::new("bb0".into(), "bb2".into(), "return".into()),
            Edge::new("bb2".into(), "bb3".into(), "return".into()),
            Edge::new("bb2".into(), "bb4".into(), "unwind".into()),
            Edge::new("bb3".into(), "bb5".into(), "".into()),
            Edge::new("bb4".into(), "bb1".into(), "return".into()),
            Edge::new("bb5".into(), "bb6".into(), "return".into()),
            Edge::new("bb5".into(), "bb4".into(), "unwind".into()),
            Edge::new("bb6".into(), "bb7".into(), "0_isize".into()),
            Edge::new("bb6".into(), "bb9".into(), "1_isize".into()),
            Edge::new("bb6".into(), "bb8".into(), "otherwise".into()),
            Edge::new("bb7".into(), "bb15".into(), "return".into()),
            Edge::new("bb9".into(), "bb10".into(), "success".into()),
            Edge::new("bb9".into(), "bb4".into(), "unwind".into()),
            Edge::new("bb10".into(), "bb12".into(), "0_i32".into()),
            Edge::new("bb10".into(), "bb11".into(), "otherwise".into()),
            Edge::new("bb11".into(), "bb14".into(), "".into()),
            Edge::new("bb12".into(), "bb13".into(), "return".into()),
            Edge::new("bb12".into(), "bb4".into(), "unwind".into()),
            Edge::new("bb13".into(), "bb14".into(), "".into()),
            Edge::new("bb14".into(), "bb5".into(), "".into())
        ],
    )
}

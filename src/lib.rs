//! **G**eneric **S**tringly typed **G**raph **D**ata**T**ype
//!
//! Create graphs which have a list of strings as nodes and compute the diff between them.
//!
//! Use [match_graphs](fn.match_graphs.html) to get the mapping (isomorphism) between the nodes.
//!
//! Use [visualize_diff](fn.visualize_diff.html) to get a [MultiGraph](struct.MultiGraph.html)
//! which can then be converted into dot using the [to_dot](struct.MultiGraph.html#method.to_dot)
//! method. The dot file generated can be rendered to various formats using graphviz.
//!
//! ```
//! use gsgdt::*;
//!
//! let label1: String = "bb0__0_3".into();
//! let label2: String = "bb0__1_3".into();
//! let style: NodeStyle = Default::default();
//!
//! let nodes = vec![
//!     Node::from_list(
//!         vec!["_1 = const 1_i32".into(), "_2 = const 2_i32".into()],
//!         label1.clone(),
//!         "0".into(),
//!         style.clone(),
//!     ),
//!     Node::from_list(
//!         vec!["return".into()],
//!         label2.clone(),
//!         "1".into(),
//!         style.clone(),
//!     ),
//! ];
//!
//! let g = Graph::new(
//!     "Mir_0_3".into(),
//!     nodes,
//!     vec![Edge::new(label1, label2, "return".into())],
//! );
//! ```
//!
#![allow(rustc::default_hash_types)]
mod diff;
mod graph;
mod levenshtein;
mod multi_graph;
mod node;
mod util;

pub use diff::*;
pub use graph::*;
pub use multi_graph::*;
pub use node::*;

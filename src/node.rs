use crate::util::escape_html;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// NodeStyle defines some style of [Node](struct.Node.html)
#[derive(Clone, Serialize, Deserialize)]
pub struct NodeStyle {
    /// Override the title color of the title
    /// To color the title of the node differently in graphviz
    pub title_bg: Option<String>,

    /// Print a seperator b/w the rest of the statements and the last one
    pub last_stmt_sep: bool,
}

impl Default for NodeStyle {
    fn default() -> NodeStyle {
        NodeStyle {
            title_bg: None,
            last_stmt_sep: false,
        }
    }
}

/// A graph node
#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    /// A list of statements.
    pub content: String,

    /// A unique identifier for the given node.
    pub label: String,

    /// The title is printed on the top of BB, the index of the basic block
    pub(crate) title: String,

    /// Can be used to override the default styles
    pub(crate) style: NodeStyle,

    pub(crate) content_length: usize,
}

impl Node {
    // TODO: rename to from_list
    pub fn new(stmts: Vec<String>, label: String, title: String, style: NodeStyle) -> Node {
        let stmts_len = stmts.len();
        let mut transformed_stmts = Vec::with_capacity(stmts_len);
        if !stmts.is_empty() {
            if stmts.len() > 1 {
                transformed_stmts.push(r#"<tr><td align="left" balign="left">"#.to_owned());
                for statement in &stmts[..stmts_len - 1] {
                    transformed_stmts.push(format!("{}<br/>", escape_html(statement)));
                }
                transformed_stmts.push("</td></tr>".to_owned());
            }

            if !style.last_stmt_sep {
                transformed_stmts.push(format!(
                    r#"<tr><td align="left">{}</td></tr>"#,
                    escape_html(&stmts[stmts_len - 1])
                ));
            } else {
                transformed_stmts.push(format!(
                    r#"<tr><td align="left" balign="left">{}</td></tr>"#,
                    escape_html(&stmts[stmts_len - 1])
                ));
            }
        }
        let content = transformed_stmts.join("");
        let content_length = Self::length_heuristic(&content);
        Node {
            content,
            label,
            title,
            style,
            content_length,
        }
    }

    /// Returns an approximate measure of the length of the node content.
    /// This is an approximate measure used to improve the matching.
    /// This function simply counts the number of `</br>` and `<tr>` in the content html
    /// and returns it.
    fn length_heuristic(content: &str) -> usize {
        content.matches("<br/>").count() + content.matches("<tr>").count()
    }

    pub fn get_content_length(&self) -> usize {
        self.content_length
    }

    pub fn to_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, r#"<table border="0" cellborder="1" cellspacing="0">"#)?;

        let bg_attr = match &self.style.title_bg {
            Some(color) => format!(r#"bgcolor="{}""#, color),
            None => "".into(),
        };
        write!(
            w,
            r#"<tr><td {bg_attr} {attrs} colspan="{colspan}">{blk}</td></tr>"#,
            attrs = r#"align="center""#,
            // TODO: Not sure what this is for
            colspan = 1,
            blk = self.title,
            bg_attr = bg_attr
        )?;

        write!(w, "{}", self.content)?;
        // if !self.stmts.is_empty() {
        //     if self.stmts.len() > 1 {
        //         write!(w, r#"<tr><td align="left" balign="left">"#)?;
        //         for statement in &self.stmts[..stmts_len - 1] {
        //             write!(w, "{}<br/>", escape_html(statement))?;
        //         }
        //         write!(w, "</td></tr>")?;
        //     }
        //
        //     if !self.style.last_stmt_sep {
        //         write!(w, r#"<tr><td align="left">"#)?;
        //         write!(w, "{}", escape_html(&self.stmts[stmts_len - 1]))?;
        //     } else {
        //         write!(w, r#"<tr><td align="left" balign="left">"#)?;
        //         write!(w, "{}", escape_html(&self.stmts[stmts_len - 1]))?;
        //     }
        //     write!(w, "</td></tr>")?;
        // }

        write!(w, "</table>")
    }
}

/// EdgeStyle defines some style of [Edge](struct.Edge.html)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EdgeStyle {
    /// Override the color of the edge.
    pub color: Option<String>,
}

impl Default for EdgeStyle {
    fn default() -> EdgeStyle {
        EdgeStyle { color: None }
    }
}
/// A directed graph edge
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Edge {
    /// The label of the source node of the edge.
    pub from: String,

    /// The label of the target node of the edge.
    pub to: String,

    /// The label (title) of the edge. This doesn't have to unique.
    // TODO: Rename this to title?
    pub label: String,

    pub style: EdgeStyle,
}

impl Edge {
    pub fn new(from: String, to: String, label: String) -> Edge {
        Edge {
            from,
            to,
            label,
            style: Default::default(),
        }
    }

    pub fn to_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut attrs = format!(r#"label="{}""#, self.label);
        if let Some(color) = &self.style.color {
            attrs.push_str(&format!(r#" color="{}""#, color));
        }
        writeln!(w, r#"    {} -> {} [{}];"#, self.from, self.to, attrs)
    }
}

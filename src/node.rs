use std::io::{self, Write};
use crate::util::escape_html;

#[derive(Clone)]
pub struct NodeStyle {
    // Override the title color of the title
    // To color the title of the node differently in graphviz
    pub title_bg: Option<String>,

    // Print a seperator b/w the rest of the statements and the last one
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

pub struct Node {
    pub stmts: Vec<String>,
    pub label: String,

    // The title is printed on the top of BB, the index of the basic block
    title: String,
    // Can be used to override the default styles
    style: NodeStyle,
}

impl Node {
    pub fn new(stmts: Vec<String>, label: String, title: String, style: NodeStyle) -> Node {
        Node {
            stmts,
            label,
            title,
            style,
        }
    }

    pub fn to_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, r#"<table border="0" cellborder="1" cellspacing="0">"#)?;

        let bg_attr = match &self.style.title_bg {
            Some(color) => format!("bgcolor={}", color),
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

        let stmts_len = self.stmts.len();
        if !self.stmts.is_empty() {
            write!(w, r#"<tr><td align="left" balign="left">"#)?;
            for statement in &self.stmts[..stmts_len-1] {
                write!(w, "{}<br/>", escape_html(statement))?;
            }
            write!(w, "</td></tr>")?;
        }

        if !self.style.last_stmt_sep {
            write!(w, r#"<tr><td balign="left">"#)?;
            write!(w, "{}<br/>", escape_html(&self.stmts[stmts_len-1]))?;
            write!(w, "</td></tr>")?;
        } else {
            write!(w, r#"<tr><td align="left" balign="left">"#)?;
            write!(w, "{}<br/>", escape_html(&self.stmts[stmts_len-1]))?;
            write!(w, "</td></tr>")?;
        }

        // TODO: add a seperator for the last instr (terminator)

        write!(w, "</table>")
    }
}

pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: String,
}

impl Edge {
    pub fn new(from: String, to: String, label: String) -> Edge {
        Edge { from, to, label }
    }
    pub fn to_dot<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(
            w,
            r#"    {} -> {} [label="{}"];"#,
            self.from, self.to, self.label
        )
    }
}


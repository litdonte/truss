use std::fmt::Display;

use crate::core::html::node::HtmlNode;

mod attribute;
pub mod node;

pub struct Html<'a> {
    root: HtmlNode<'a>,
}

impl<'a> Html<'a> {
    pub fn new(root: HtmlNode<'a>) -> Self {
        Html { root }
    }
}

impl Display for Html<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

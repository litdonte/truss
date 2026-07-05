use std::fmt::Display;

use crate::core::dom::html::attribute::{HtmlAttribute, HtmxAttribute};

pub enum HtmlNode<'a> {
    Element {
        tag: &'static str,
        attributes: Vec<HtmlAttribute>,
        htmx_attributes: Vec<HtmxAttribute<'a>>,
        children: Vec<HtmlNode<'a>>,
    },
    Text(String),
}

impl Display for HtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::Element {
                tag,
                attributes,
                htmx_attributes,
                children,
            } => {
                let html_attributes = attributes
                    .iter()
                    .map(|attr| attr.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                let htmx_attributes = htmx_attributes
                    .iter()
                    .map(|attr| attr.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                let children = children
                    .iter()
                    .map(|child| child.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                let all_attrs = [html_attributes, htmx_attributes]
                    .iter()
                    .filter(|attrs| !attrs.is_empty())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");

                write!(
                    f,
                    "<{}{}>{}</{}>",
                    tag,
                    if all_attrs.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", all_attrs)
                    },
                    children,
                    tag
                )
            }
        }
    }
}

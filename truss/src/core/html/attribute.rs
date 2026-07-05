use std::fmt::Display;

use crate::core::{component::Targetable, route::RouteInfo};

pub enum HtmlAttribute {
    Class(Vec<String>),
    Id(String),
    Style(String),
    Data(String, String),
}

impl Display for HtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class(classes) => write!(f, "class=\"{}\"", classes.join(" ")),
            Self::Id(id) => write!(f, "id=\"{}\"", id),
            Self::Style(style) => write!(f, "style=\"{}\"", style),
            Self::Data(key, value) => write!(f, "data-{}=\"{}\"", key, value),
        }
    }
}

pub enum HtmxAttribute<'a> {
    HxGet(Box<dyn RouteInfo>),
    HxPost(Box<dyn RouteInfo>),
    HxPut(Box<dyn RouteInfo>),
    HxPatch(Box<dyn RouteInfo>),
    HxDelete(Box<dyn RouteInfo>),
    HxTarget(&'a dyn Targetable),
    HxSwap(String),
    HxTrigger(String),
}

impl Display for HtmxAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HxGet(route) => write!(f, "hx-get=\"{}\"", route.path()),
            Self::HxPost(route) => write!(f, "hx-post=\"{}\"", route.path()),
            Self::HxPut(route) => write!(f, "hx-put=\"{}\"", route.path()),
            Self::HxPatch(route) => write!(f, "hx-patch=\"{}\"", route.path()),
            Self::HxDelete(route) => write!(f, "hx-delete=\"{}\"", route.path()),
            Self::HxTarget(target) => write!(f, "hx-target=\"#{}\"", target.target_id()),
            Self::HxSwap(swap) => write!(f, "hx-swap=\"{}\"", swap),
            Self::HxTrigger(trigger) => write!(f, "hx-trigger=\"{}\"", trigger),
        }
    }
}

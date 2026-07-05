use crate::core::{Id, html::Html};

pub trait Component {
    type Element;

    fn id(&self) -> &Id<Self::Element>;
    fn with_id(self, id: &str) -> Self
    where
        Self: Sized;
    fn render(&self) -> Html;
}

pub trait Targetable {
    fn target_id(&self) -> &str;
}

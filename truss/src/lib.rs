pub mod core;

pub mod prelude {
    pub use crate::core::Id;
    pub use crate::core::component::Component;
    pub use crate::core::html::Html;
    pub use crate::core::route::HttpMethod::*;
    pub use crate::core::route::RouteInfo;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::core::html::Html;
    use crate::core::query::IntoQueryParam;
    use crate::core::{component::Component, html::node::HtmlNode};
    use truss_macros::{component, query};

    struct Scene {
        title: String,
    }

    #[component]
    pub struct ScenePanel {
        scene: Scene,
    }

    impl ScenePanel {
        fn render(&self) -> Html<'_> {
            Html::new(HtmlNode::Text(self.scene.title.clone()))
        }
    }

    #[query]
    pub enum SceneQuery {
        Character(String),
    }

    #[test]
    fn test_component_has_id() {
        let panel = ScenePanel::new(Scene {
            title: "Act 1".to_string(),
        });
        // id exists and is not empty
        assert!(!panel.id().value().is_empty());
    }

    #[test]
    fn test_component_with_id() {
        let panel = ScenePanel::new(Scene {
            title: "Act 1".to_string(),
        })
        .with_id("primary-panel");
        // id starts with the supplied prefix
        assert!(panel.id().value().starts_with("primary-panel"));
    }

    #[test]
    fn test_query_has_correct_key() {
        let query = SceneQuery::Character("Amelia".to_string());
        assert_eq!(query.key(), "character".to_string());
    }

    #[test]
    fn test_query_has_correct_value() {
        let query = SceneQuery::Character("Amelia".to_string());
        assert_eq!(query.value(), "Amelia".to_string());
    }
}

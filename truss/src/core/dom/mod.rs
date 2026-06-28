use std::fmt::Display;

pub struct Html {
    tree: String,
}

impl Html {
    pub fn new(content: impl Into<String>) -> Self {
        Html {
            tree: content.into(),
        }
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tree)
    }
}

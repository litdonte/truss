pub mod component;
pub mod html;
pub mod query;
pub mod route;

use std::marker::PhantomData;

use ulid::Ulid;

pub struct Id<T> {
    value: String,
    _kind: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            value: Ulid::new().to_string(),
            _kind: PhantomData,
        }
    }

    pub fn new_with(prefix: &str) -> Self {
        Self {
            value: format!("{}-{}", prefix, Ulid::new().to_string()),
            _kind: PhantomData,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<T> AsRef<str> for Id<T> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

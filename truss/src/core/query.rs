pub trait IntoQueryParam {
    fn key(&self) -> String;
    fn value(&self) -> String;
}

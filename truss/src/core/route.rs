pub enum HttpMethod<R: Route> {
    Get(R),
    Post(R),
    Put(R),
    Patch(R),
    Delete(R),
}

pub trait Route {
    fn method(&self) -> HttpMethod<Self>
    where
        Self: Sized;
    fn path(&self) -> String;
}

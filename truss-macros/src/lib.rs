use proc_macro::TokenStream;

mod component;
mod helpers;
mod query;
mod routes;

#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    component::expand(_args, input)
}

#[proc_macro_attribute]
pub fn query(_args: TokenStream, input: TokenStream) -> TokenStream {
    query::expand(_args, input)
}

#[proc_macro_attribute]
pub fn routes(_args: TokenStream, input: TokenStream) -> TokenStream {
    routes::expand(_args, input)
}

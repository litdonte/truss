use proc_macro::TokenStream;

mod component;
mod element;
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

#[proc_macro]
pub fn fragment(input: TokenStream) -> TokenStream {
    element::expand_fragment(input)
}

#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    element::expand_element("div", input)
}

#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    element::expand_element("button", input)
}

#[proc_macro]
pub fn span(input: TokenStream) -> TokenStream {
    element::expand_element("span", input)
}

#[proc_macro]
pub fn p(input: TokenStream) -> TokenStream {
    element::expand_element("p", input)
}

#[proc_macro]
pub fn a(input: TokenStream) -> TokenStream {
    element::expand_element("a", input)
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    element::expand_element("form", input)
}

#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    element::expand_element("input", input)
}

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    element::expand_element("head", input)
}

#[proc_macro]
pub fn body(input: TokenStream) -> TokenStream {
    element::expand_element("body", input)
}

#[proc_macro]
pub fn title(input: TokenStream) -> TokenStream {
    element::expand_element("title", input)
}

#[proc_macro]
pub fn meta(input: TokenStream) -> TokenStream {
    element::expand_element("meta", input)
}

#[proc_macro]
pub fn link(input: TokenStream) -> TokenStream {
    element::expand_element("link", input)
}
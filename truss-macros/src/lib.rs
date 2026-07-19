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

#[proc_macro]
pub fn h1(input: TokenStream) -> TokenStream {
    element::expand_element("h1", input)
}

#[proc_macro]
pub fn h2(input: TokenStream) -> TokenStream {
    element::expand_element("h2", input)
}

#[proc_macro]
pub fn h3(input: TokenStream) -> TokenStream {
    element::expand_element("h3", input)
}

#[proc_macro]
pub fn h4(input: TokenStream) -> TokenStream {
    element::expand_element("h4", input)
}

#[proc_macro]
pub fn h5(input: TokenStream) -> TokenStream {
    element::expand_element("h5", input)
}

#[proc_macro]
pub fn h6(input: TokenStream) -> TokenStream {
    element::expand_element("h6", input)
}

#[proc_macro]
pub fn blockquote(input: TokenStream) -> TokenStream {
    element::expand_element("blockquote", input)
}

#[proc_macro]
pub fn pre(input: TokenStream) -> TokenStream {
    element::expand_element("pre", input)
}

#[proc_macro]
pub fn code(input: TokenStream) -> TokenStream {
    element::expand_element("code", input)
}

#[proc_macro]
pub fn strong(input: TokenStream) -> TokenStream {
    element::expand_element("strong", input)
}

#[proc_macro]
pub fn em(input: TokenStream) -> TokenStream {
    element::expand_element("em", input)
}

#[proc_macro]
pub fn br(input: TokenStream) -> TokenStream {
    element::expand_element("br", input)
}

#[proc_macro]
pub fn hr(input: TokenStream) -> TokenStream {
    element::expand_element("hr", input)
}

#[proc_macro]
pub fn ul(input: TokenStream) -> TokenStream {
    element::expand_element("ul", input)
}

#[proc_macro]
pub fn ol(input: TokenStream) -> TokenStream {
    element::expand_element("ol", input)
}

#[proc_macro]
pub fn li(input: TokenStream) -> TokenStream {
    element::expand_element("li", input)
}
#[proc_macro]
pub fn header(input: TokenStream) -> TokenStream {
    element::expand_element("header", input)
}

#[proc_macro]
pub fn footer(input: TokenStream) -> TokenStream {
    element::expand_element("footer", input)
}

#[proc_macro]
pub fn main(input: TokenStream) -> TokenStream {
    element::expand_element("main", input)
}

#[proc_macro]
pub fn nav(input: TokenStream) -> TokenStream {
    element::expand_element("nav", input)
}

#[proc_macro]
pub fn section(input: TokenStream) -> TokenStream {
    element::expand_element("section", input)
}

#[proc_macro]
pub fn article(input: TokenStream) -> TokenStream {
    element::expand_element("article", input)
}

#[proc_macro]
pub fn aside(input: TokenStream) -> TokenStream {
    element::expand_element("aside", input)
}

#[proc_macro]
pub fn img(input: TokenStream) -> TokenStream {
    element::expand_element("img", input)
}

#[proc_macro]
pub fn video(input: TokenStream) -> TokenStream {
    element::expand_element("video", input)
}

#[proc_macro]
pub fn audio(input: TokenStream) -> TokenStream {
    element::expand_element("audio", input)
}

#[proc_macro]
pub fn table(input: TokenStream) -> TokenStream {
    element::expand_element("table", input)
}

#[proc_macro]
pub fn thead(input: TokenStream) -> TokenStream {
    element::expand_element("thead", input)
}

#[proc_macro]
pub fn tbody(input: TokenStream) -> TokenStream {
    element::expand_element("tbody", input)
}

#[proc_macro]
pub fn tr(input: TokenStream) -> TokenStream {
    element::expand_element("tr", input)
}

#[proc_macro]
pub fn th(input: TokenStream) -> TokenStream {
    element::expand_element("th", input)
}

#[proc_macro]
pub fn td(input: TokenStream) -> TokenStream {
    element::expand_element("td", input)
}
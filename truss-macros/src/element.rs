use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Lit, Token, parse::Parser, punctuated::Punctuated};

enum ElementContent<'a> {
    Text(String),
    HtmlAttr(&'a Expr),
    HtmxAttr(&'a Expr),
    Child(&'a Expr),
}

fn categorize(expr: &Expr) -> ElementContent<'_> {
    match expr {
        Expr::Lit(expr_lit) => {
            let Lit::Str(s) = &expr_lit.lit else {
                panic!("only string literals are supported as text content");
            };
            ElementContent::Text(s.value())
        }
        Expr::Call(call) => {
            if let Expr::Path(expr_path) = call.func.as_ref() {
                let name = expr_path
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default();

                match name.as_str() {
                    "HxGet" | "HxPost" | "HxPut" | "HxPatch" | "HxDelete" | "HxTarget"
                    | "HxSwap" | "HxTrigger" => ElementContent::HtmxAttr(expr),
                    "Class" | "Id" | "Style" | "Data" => ElementContent::HtmlAttr(expr),
                    _ => ElementContent::Child(expr),
                }
            } else {
                ElementContent::Child(expr)
            }
        }
        _ => ElementContent::Child(expr),
    }
}

pub fn expand_fragment(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let expressions = parser
        .parse(input)
        .expect("failed to parse fragment content");

    let children: Vec<proc_macro2::TokenStream> = expressions
        .iter()
        .map(|expr| match expr {
            Expr::Lit(lit_expr) => {
                let Lit::Str(s) = &lit_expr.lit else {
                    panic!("only string literals are supported in fragment!")
                };
                quote! { crate::core::html::node::HtmlNode::Text(#s.to_string()) }
            }
            _ => quote! {#expr},
        })
        .collect();

    quote! {
        crate::core::html::node::HtmlNode::Fragment(vec![#(#children),*])
    }
    .into()
}

pub fn expand_element(tag: &str, input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let expressions = parser
        .parse(input)
        .expect("failed to parse element content");

    let mut html_attrs: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut htmx_attrs: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut children: Vec<proc_macro2::TokenStream> = Vec::new();

    expressions.iter().for_each(|expr| match categorize(expr) {
        ElementContent::HtmlAttr(ex) => html_attrs.push(quote! {#ex}),
        ElementContent::HtmxAttr(ex) => htmx_attrs.push(quote! {#ex}),
        ElementContent::Child(ex) => children.push(quote! {#ex}),
        ElementContent::Text(s) => {
            children.push(quote! {crate::core::html::HtmlNode::Text(#s.to_string())})
        }
    });

    quote! {
        crate::core::html::HtmlNode::Element {
            tag: #tag,
            attributes: vec![#(#html_attrs),*],
            htmx_attributes: vec![#(#htmx_attrs),*],
            children: vec![#(#children),*],
        }
    }
    .into()
}

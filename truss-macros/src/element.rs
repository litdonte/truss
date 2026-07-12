use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Lit, Token, parse::Parser, punctuated::Punctuated};

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

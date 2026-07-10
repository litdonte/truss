use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

use crate::helpers::{VariantInfo, capitalize};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input enum
    let parsed_input = parse_macro_input!(input as DeriveInput);

    // Extract enum name and variants
    let enum_name = &parsed_input.ident;
    let variants: Vec<_> = match &parsed_input.data {
        Data::Enum(data) => data.variants.iter().collect(),
        _ => panic!("#[routes] can only be applied to enums"),
    };

    // Extract HTTP method, path, and variant info from attributes
    let parsed_variants: Vec<_> = variants
        .iter()
        .map(|v| {
            v.attrs
                .iter()
                .find_map(|at| {
                    let method = at.path().get_ident().map(|i| i.to_string())?;
                    matches!(method.as_str(), "get" | "post" | "put" | "patch" | "delete").then(
                        || VariantInfo {
                            name: &v.ident,
                            method: capitalize(&method),
                            path: at.parse_args::<syn::LitStr>().unwrap().value(),
                            is_unit: matches!(v.fields, syn::Fields::Unit),
                        },
                    )
                })
                .unwrap_or_else(|| panic!("variant missing HTTP method attribute"))
        })
        .collect();

    // Generate match patterns (unit vs tuple variants)
    let variant_patterns: Vec<proc_macro2::TokenStream> = parsed_variants
        .iter()
        .map(|v| {
            let name = v.name;
            if v.is_unit {
                quote! { #enum_name::#name }
            } else {
                quote! { #enum_name::#name(_) }
            }
        })
        .collect();

    // Convert method strings to Ident tokens for code generation
    let variant_methods: Vec<_> = parsed_variants
        .iter()
        .map(|v| {
            let method = &v.method; // "Get", "Post" etc
            syn::Ident::new(method, proc_macro2::Span::call_site())
        })
        .collect();

    // Extract paths for code generation
    let variant_paths: Vec<_> = parsed_variants.iter().map(|v| &v.path).collect();

    // Strip HTTP method attributes from variants before re-emitting
    let clean_variants: Vec<_> = variants
        .iter()
        .map(|v| {
            let mut clean = (*v).clone(); // dereference and clone to get owned value
            clean.attrs.retain(|a| {
                !matches!(
                    a.path().get_ident().map(|i| i.to_string()).as_deref(),
                    Some("get" | "post" | "put" | "patch" | "delete")
                )
            });
            clean
        })
        .collect();

    // Generate the cleaned enum and RouteInfo impl
    quote! {
        pub enum #enum_name {
            #(#clean_variants),*
        }


        impl crate::core::route::RouteInfo for #enum_name {
            fn method(&self) -> HttpMethod {
                match self {
                    #(#variant_patterns => crate::core::route::HttpMethod::#variant_methods),*
                }
            }

            fn path(&self) -> &'static str {
                match self {
                    #(#variant_patterns => #variant_paths),*
                }
            }
        }

    }
    .into()
}

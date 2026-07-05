use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::helpers::{VariantInfo, capitalize};

mod helpers;

#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields: Vec<_> = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect(),
            _ => panic!("#[component] only supports structs with named fields"),
        },
        _ => panic!("#[component] can only be applied to structs"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    quote! {
        pub struct #struct_name {
            #(#fields,)*
            instance_id: crate::core::Id<#struct_name>,
        }

        impl #struct_name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names,)*
                    instance_id: crate::core::Id::default(),
                }
            }
        }

        impl crate::core::component::Component for #struct_name {
            type Element = #struct_name;
            fn id(&self) -> &crate::core::Id<#struct_name> {
                &self.instance_id
            }
            fn render(&self) -> crate::core::dom::Html {
                crate::core::dom::Html::new(String::new())
            }
            fn with_id(mut self, id: &str) -> Self where Self:Sized {
                self.instance_id = crate::core::Id::new_with(id);
                self
            }
        }

        impl crate::core::component::Targetable for #struct_name {
            fn target_id(&self) -> &str {
                self.instance_id.value()
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn query(_args: TokenStream, input: TokenStream) -> TokenStream {
    let cloned_input = input.clone();
    let parsed_input = parse_macro_input!(input as DeriveInput);

    let enum_name = &parsed_input.ident;

    let variants: Vec<_> = match &parsed_input.data {
        Data::Enum(data) => data.variants.iter().collect(),
        _ => panic!("#[query] can only be applied to enums"),
    };

    let variant_keys: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let input = proc_macro2::TokenStream::from(cloned_input);

    quote! {
        #input

        impl crate::core::query::IntoQueryParam for #enum_name {
            fn key(&self) -> String {
                match self {
                    #(#enum_name::#variant_keys(_) => stringify!(#variant_keys).to_lowercase()),*
                }            }
            fn value(&self) -> String {
                match self {
                    #(#enum_name::#variant_keys(v) => v.to_string()),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn routes(_args: TokenStream, input: TokenStream) -> TokenStream {
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

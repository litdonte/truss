use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
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

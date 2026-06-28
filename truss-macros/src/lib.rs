use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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
                    instance_id: crate::core::Id::new(),
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

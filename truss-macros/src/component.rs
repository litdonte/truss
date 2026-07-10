use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
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
            fn render(&self) -> crate::core::html::Html {
                crate::core::html::Html::new(HtmlNode::Text(String::from(stringify!(#struct_name))))
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

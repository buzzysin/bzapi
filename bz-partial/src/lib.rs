use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Type, parse_macro_input};

#[proc_macro_derive(Partial)]
pub fn partial_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Partial derive only supports named fields"),
        },
        _ => panic!("Partial derive only supports structs"),
    };

    let partial_type_name = Ident::new(&format!("{}Partial", name), name.span());

    let partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        let partial_type = match field_type {
            Type::Path(type_path)
                if type_path
                    .path
                    .segments
                    .last()
                    .map(|seg| seg.ident.to_string())
                    == Some("Option".to_string()) =>
            {
                quote! { #field_type } // Already an Option, keep as is
            }
            _ => quote! { Option<#field_type> }, // Wrap in Option
        };

        quote! {
            #field_name: #partial_type
        }
    });

    let to_partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: self.#field_name.clone()
        }
    });

    let merge_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: partial.#field_name.clone().or_else(|| self.#field_name.clone())
        }
    });

    let output = quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #partial_type_name {
            #(#partial_fields,)*
        }

        impl Partial for #name {
            type PartialSelf = #partial_type_name;

            fn to_partial(&self) -> Self::PartialSelf {
                #partial_type_name {
                    #(#to_partial_fields,)*
                }
            }

            fn merge(&self, partial: Self::PartialSelf) -> Self {
                Self {
                    #(#merge_fields,)*
                }
            }
        }
    };

    TokenStream::from(output)
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
    t.compile_fail("tests/ui-fail/*.rs");
}

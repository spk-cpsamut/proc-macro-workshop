use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, parse_macro_input, DeriveInput, Ident, Type};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let ident_builder = format_ident!("{}Builder", &input.ident);
    let mut field_names: Vec<Ident> = vec![];
    let mut field_types: Vec<Type> = vec![];

    if let syn::Data::Struct(data_struct) = &input.data {
        for field in &data_struct.fields {
            field_names.push(field.clone().ident.unwrap());
            field_types.push(field.clone().ty);
        }
    }

    let builder_struct = quote! {
        pub struct #ident_builder {
            #(#field_names: Option<#field_types>),*
        }
    };

    let builder_default = quote! {
        #ident_builder {
            #(#field_names: None),*
        }
    };

    let setter_methods = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let value_build_method = field_names.iter().map(|name| {
        quote! {
            #name: self.#name.clone().unwrap(),
        }
    });

    let expanded = quote! {
        #builder_struct

        impl #ident {
            pub fn builder() -> #ident_builder {
                #builder_default
            }
        }

        impl #ident_builder {
            #(#setter_methods)*
            
            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#value_build_method)*
                })
            }
        }
    };
    TokenStream::from(expanded)
}

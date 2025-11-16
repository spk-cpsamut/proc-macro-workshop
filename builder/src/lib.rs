use proc_macro::{ TokenStream};
use syn::{self, DeriveInput, Fields, Ident, Type, parse_macro_input};
use quote::{format_ident, quote};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let iden = input.ident;
    let build = format_ident!("{}Builder", &iden);

    let mut fields_vec: Vec<Ident> = vec!();
    let mut ty_vec: Vec<Type> = vec!();
    if let syn::Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                fields_vec.push(field.clone().ident.unwrap());
                ty_vec.push(field.clone().ty)
            }
        }
    }
    let expaned = quote!{
        struct #build {
            #(#fields_vec: Option<#ty_vec>),*
        } 
        impl #iden {
            pub fn builder() -> #build {
                #build {
                    #(#fields_vec: None),*
                }
            }
        }
    };

    TokenStream::from(expaned)
}

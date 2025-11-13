use proc_macro::TokenStream;
use syn::{self, DeriveInput, parse_macro_input};
use quote::quote;
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expaned = quote!{};

    TokenStream::from(expaned)
}

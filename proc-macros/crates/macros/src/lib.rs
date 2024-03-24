#![allow(unused)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyProc, attributes(inst))]
pub fn my_proc_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            fn hi(&self) {
                println!("hello from {}, {}", stringify!(#name), self.name);
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn as_is(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

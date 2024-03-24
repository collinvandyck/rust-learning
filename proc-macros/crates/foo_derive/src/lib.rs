#![allow(unused)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Foo, attributes(inst))]
pub fn my_proc_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    // eprintln!("ast: {ast:#?}");
    let name = ast.ident;
    let syn::Data::Struct(ref st) = ast.data else {
        panic!("must be a struct")
    };
    let expanded = quote! {
        impl #name {
            fn hi(&self) {
                println!("hello from {}, {}", stringify!(#name), self.name);
            }
        }

        /// This is a doc comment
        impl Foo for #name {

            /// a foo does a foo
            fn foo(&self) -> String {
                String::from("foooo")
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn as_is(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

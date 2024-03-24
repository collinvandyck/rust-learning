#![allow(unused)]

extern crate proc_macro;
use proc_macro::TokenStream;

mod derive;

#[proc_macro_derive(MyProc)]
pub fn my_proc_derive(_item: TokenStream) -> TokenStream {
    println!("my_proc_derive: {_item}");
    "fn my_proc_derive() -> u32 { 43 }".parse().unwrap()
}

#[proc_macro]
pub fn my_proc(_item: TokenStream) -> TokenStream {
    "fn my_proc() -> u32 { 41 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn as_is(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

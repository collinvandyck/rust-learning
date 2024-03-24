#![allow(unused)]

extern crate proc_macro;
use proc_macro::TokenStream;

mod derive;

#[proc_macro_derive(MyProc, attributes(my_proc))]
pub fn my_proc_derive(_item: TokenStream) -> TokenStream {
    println!("my_proc_derive: {_item}");
    "fn my_proc_derive() -> u32 { 43 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn as_is(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

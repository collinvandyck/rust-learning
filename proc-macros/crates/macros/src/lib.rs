extern crate proc_macro;
use proc_macro::TokenStream;

mod derive;

#[proc_macro]
pub fn make_answer_pm(_item: TokenStream) -> TokenStream {
    derive::make_answer_pm(_item)
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(Barks)]
pub fn derive_barks(ts: TokenStream) -> TokenStream {
    ts.into_iter().for_each(|tok: proc_macro::TokenTree| {
        println!("{tok:?}");
    });
    "fn barks() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn as_is(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("attr: {attr:#?}");
    println!("item: \"{item}\"");
    item
}

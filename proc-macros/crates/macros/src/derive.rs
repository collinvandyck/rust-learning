use super::*;

pub fn my_proc(_item: TokenStream) -> TokenStream {
    "fn my_proc() -> u32 { 41 }".parse().unwrap()
}

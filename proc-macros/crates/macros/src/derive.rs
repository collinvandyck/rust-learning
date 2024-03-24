use super::*;

pub(crate) fn make_answer_pm(_item: TokenStream) -> TokenStream {
    "fn answer_pm() -> u32 { 41 }".parse().unwrap()
}

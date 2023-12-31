mod hashmap;

mod prelude {
    pub use crate::hashmap::*;
}

use prelude::*;

fn main() {
    let mut h = HashMap::new();
    h.add("foo", "bar");
    dbg!(h.get("foo"));
    dbg!(h.get("bar"));
}

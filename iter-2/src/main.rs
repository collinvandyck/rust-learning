use std::vec;

fn main() {
    let v = vec!["a", "b", "c", "d", "e"];
    let r1 = (0..v.len() - 1).rev();
    let r2 = (0..v.len()).rev();
    let iter = std::iter::zip(r1, r2);
    iter.for_each(|x| println!("{x:?}"));
}

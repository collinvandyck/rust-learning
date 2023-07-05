use std::ops::Deref;

fn main() {
    let things = Things::new(&[1, 2, 3]);
    for x in things.iter() {
        println!("{x}")
    }
}

struct Things<T>(Vec<T>);

impl<T> Things<T>
where
    T: Clone + Copy,
{
    fn new(vals: &[T]) -> Self {
        Self(vals.into())
    }
}

impl<T> Deref for Things<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

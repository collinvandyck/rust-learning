fn main() {
    let v = vec![vec![2, 3], vec![4, 5, 6]];
    let f = Flatten::new(v.into_iter());
    for x in f {
        println!("{x}");
    }
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|inner| inner.into_iter().next())
    }
}

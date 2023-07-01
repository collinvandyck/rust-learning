fn main() {
    let x = triangle(5);
    println!("X: {}", x);

    let vals = Values::from(vec![1, 2, 3]);
    for x in vals {
        println!("x: {}", x)
    }
}

fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

struct Values<T> {
    vals: Vec<T>,
    idx: usize,
}

impl<T> From<Vec<T>> for Values<T> {
    fn from(vals: Vec<T>) -> Self {
        Values { vals, idx: 0 }
    }
}

impl<T> Iterator for Values<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.vals.get(self.idx);
        self.idx += 1;
        res.copied()
    }
}

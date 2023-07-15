fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
struct Num(Vec<u64>);

impl Num {
    fn new() -> Self {
        Self(vec![])
    }
    #[allow(dead_code)]
    fn from(mut v: u64) -> Self {
        let mut res = Self::new();
        let divisor: u64 = 10;
        loop {
            println!("v: {v} divisor: {divisor}");
            let part = v % divisor;
            res.insert(part);
            v = v / divisor;
            if v == 0 {
                break;
            }
        }
        res
    }
    fn insert(&mut self, v: u64) {
        self.0.insert(0, v)
    }
    fn add(&self, other: &Num) -> Self {
        let v1 = &self.0;
        let v2 = &other.0;
        for n1 in v1.iter().rev() {
            for n2 in v2.iter().rev() {}
        }
        Self::new()
    }
}

#[test]
fn test_num_from() {
    assert_eq!(Num::from(0), Num(vec![0]));
    assert_eq!(Num::from(643), Num(vec![6, 4, 3]));
    assert_eq!(
        Num::from(1234567890),
        Num(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
}

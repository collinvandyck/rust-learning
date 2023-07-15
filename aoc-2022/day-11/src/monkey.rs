#[derive(Debug)]
pub struct Monkey {
    idx: usize,
}

impl Monkey {
    pub fn load(iter: &mut impl Iterator<Item = String>) -> Option<Self> {
        let idx = Self::parse_monkey(iter.next().unwrap());
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        dbg!(Some(Self { idx }))
    }
    fn parse_monkey(input: String) -> usize {
        let parts = &input.split(' ').collect::<Vec<&str>>()[..];
        if let ["Monkey", num] = parts {
            if let Some(num) = num.split(':').next() {
                return num.parse::<usize>().unwrap();
            }
        }
        panic!("invalid monkey: {input}");
    }
}

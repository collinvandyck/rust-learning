fn main() {
    let l = List(vec![1, 2, 3]);
    l.for_each(|f| {
        println!("f: {f}");
    });

    let delta = 1;
    l.for_each(|f| {
        println!("f: {}", f + delta);
    });

    let res = l.select(|f| f % 2 == 0);
    dbg!(res);
}

struct List<T>(Vec<T>);

impl<T> List<T> {
    fn for_each<F>(&self, f: F)
    where
        F: Fn(&T),
    {
        self.0.iter().for_each(|x| {
            f(x);
        })
    }

    fn select<F>(&self, f: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut res = vec![];
        self.0.iter().for_each(|x| {
            if f(x) {
                res.push(x);
            }
        });
        res
    }
}

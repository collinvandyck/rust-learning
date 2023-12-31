use std::iter::{self, from_fn, successors};

use num::Complex;
use rand::random;

fn main() {
    let lengths: Vec<f64> = from_fn(|| {
        let lhs = random::<f64>();
        let rhs = random::<f64>();
        Some((lhs - rhs).abs())
    })
    .take(5)
    .collect();
    dbg!(lengths);

    println!("FIBS:");
    let fibs: Vec<usize> = fibonacci().take(5).collect();
    dbg!(fibs);

    let mut nums = vec![1, 2, 3];
    let foo: Vec<usize> = nums.drain(..).collect();
    dbg!(nums);
    dbg!(foo);

    successors(Some(0), |&x| if x == 0 { Some(1) } else { Some(0) })
        .take(32)
        .for_each(|x| {
            print!("{}", x);
        });
    println!();

    let name: Vec<char> = "Collin".chars().collect();
    let x: Vec<_> = name.split(|&f| f == 'l').collect();
    dbg!(x);
}

#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero: Complex<f64> = Complex::default();
    successors(Some(zero), |&z| Some(z * z + c))
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

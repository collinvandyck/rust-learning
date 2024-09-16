use std::future::Future;

#[tokio::main]
async fn main() {
    let mut futs = vec![];
    for _ in 0..5 {
        futs.push(funcit(func));
    }
    let mut count = 0;
    for f in futs {
        let fut = (f.func)(&mut count);
        let res = fut.await.unwrap();
        println!("{res}")
    }
    println!("done");
}

async fn func(num: &mut usize) -> Result<String, ()> {
    *num += 1;
    Ok(format!("{num}"))
}

struct WeirdFuture<F> {
    func: F,
}

fn funcit<'u, F, Fut>(f: F) -> WeirdFuture<F>
where
    F: Fn(&'u mut usize) -> Fut,
    Fut: Future<Output = Result<String, ()>>,
{
    WeirdFuture { func: f }
}

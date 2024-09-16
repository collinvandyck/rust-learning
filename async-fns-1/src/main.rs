#![allow(unused)]

use std::future::Future;

use futures::future::BoxFuture;

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

    let mut router = Router::default();
    for _ in 0..5 {
        router.add_route(|count| async move { RouteResult { num: count + 1 } })
    }
    router.do_route(42).await;
}

#[derive(Default)]
struct Router<'a> {
    map: Vec<ItemLife<'a>>,
}

type ItemLife<'a> = Box<dyn Fn(usize) -> BoxFuture<'a, RouteResult>>;

impl<'a> Router<'a> {
    fn add_route<F, Fut>(&mut self, handle: F)
    where
        F: Fn(usize) -> Fut + 'static,
        Fut: Future<Output = RouteResult> + 'a + Send,
    {
        self.map
            .push(Box::new(move |num: usize| Box::pin(handle(num))));
    }

    async fn do_route(&self, count: usize) {
        for m in &self.map {
            let res = m(count).await;
            println!("res = {res:?}");
        }
    }
}

#[derive(Debug)]
struct RouteResult {
    num: usize,
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

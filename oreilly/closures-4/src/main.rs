#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut f = FuncHandler {
        funcs: HashMap::new(),
    };
    f.add("/", root);
    f.add("/foo", f.foo());
}

fn root(_req: &Request) -> Response {
    Response {}
}

struct Request;
struct Response;

struct FuncHandler {
    funcs: HashMap<String, fn(&Request) -> Response>,
}

impl FuncHandler {
    fn add(&mut self, url: &str, func: fn(&Request) -> Response) {
        self.funcs.insert(url.into(), func);
    }

    fn foo<'a>(&self) -> fn(&'a Request) -> Response {
        let hello: fn(&Request) -> Response = |req| Response {};
        hello
    }
}

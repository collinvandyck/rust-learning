#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut f = funcHandler {
        funcs: HashMap::new(),
    };
    f.add("/", root);
}

fn root(req: &Request) -> Response {
    Response {}
}

struct Request;
struct Response;

struct funcHandler {
    funcs: HashMap<String, fn(&Request) -> Response>,
}

impl funcHandler {
    fn add(&mut self, url: &str, func: fn(&Request) -> Response) {
        self.funcs.insert(url.into(), func);
    }
}

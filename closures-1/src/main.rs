#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_req| get_form_response());
    router.add_route("/", |req| get_gcd_response(req));
}

fn get_gcd_response(req: &Request) -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

fn get_form_response() -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    routes: HashMap<String, C>,
}

impl<C> BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    fn new() -> BasicRouter<C> {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

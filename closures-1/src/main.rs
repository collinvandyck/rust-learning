#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_req| get_form_response());
    router.add_route("/gcd", get_gcd_response);
}

fn get_gcd_response(_req: &Request) -> Response {
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

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

fn not_found_response() -> Response {
    Response {
        code: 404,
        headers: HashMap::new(),
        body: vec![],
    }
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

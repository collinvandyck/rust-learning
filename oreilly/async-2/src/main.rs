use async_std::task;

fn main() {
    let requests = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org/wiki/Main_page".to_string(),
    ];
    let results = task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(s) => println!("*** {}\n", s),
            Err(e) => eprintln!("error: {}\n", e),
        }
    }
}

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls {
        let req = client.get(&url).recv_string();
        handles.push(task::spawn(req));
    }
    let mut results = vec![];
    for handle in handles {
        let res = handle.await;
        results.push(res);
    }
    results
}

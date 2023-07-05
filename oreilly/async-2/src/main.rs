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
            Ok(UrlInfo(url, s)) => println!("*** {url}: {} bytes", s.len()),
            Err(e) => eprintln!("error: {}\n", e),
        }
    }
}

pub struct UrlInfo(String, String);

pub async fn many_requests(urls: &[String]) -> Vec<Result<UrlInfo, surf::Exception>> {
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls {
        let fut = client.get(&url).recv_string();
        let handle = task::spawn(fut);
        handles.push((url, handle));
    }
    let mut results = vec![];
    for handle in handles {
        let (url, handle) = handle;
        let res = handle.await;
        results.push(match res {
            Ok(s) => Ok(UrlInfo(url.clone(), s)),
            Err(e) => Err(e),
        });
    }
    results
}

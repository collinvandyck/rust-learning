#![allow(unused)]

use std::{str::FromStr, time::Duration};

use anyhow::{bail, Context, Result};
use http::Extensions;
use reqwest::{Method, StatusCode, Url};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, Jitter, RetryTransientMiddleware};
use wiremock::{
    http::{HeaderMap, HeaderValue},
    matchers, Mock, MockServer, ResponseTemplate,
};

#[tokio::main]
async fn main() {}

struct Opts {
    url: Url,
    headers: HeaderMap,
}

fn new_client(opts: &Opts) -> Result<ClientWithMiddleware> {
    let delegate = reqwest::ClientBuilder::new()
        .user_agent("user-agent")
        .connection_verbose(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(1))
        .pool_idle_timeout(Duration::from_secs(1))
        .build()
        .context("could not build reqwest client")?;
    let retry_mw = RetryTransientMiddleware::new_with_policy(
        ExponentialBackoff::builder()
            // NB: it is important to use Jitter::Bounded when using retry bounds instead
            // of Jitter::Full. otherwise it is somehwat likely that you could exceed the
            // bounds when the jitter is applied, resulting in an error during the retry
            // loop.
            .jitter(Jitter::Bounded)
            .retry_bounds(Duration::from_secs(1), Duration::from_secs(5))
            .build_with_max_retries(1),
    );
    let delegate: ClientWithMiddleware = reqwest_middleware::ClientBuilder::new(delegate)
        .with(retry_mw)
        .build();
    Ok(delegate)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestBodyType {
    Streaming,
    Static,
}

async fn make_request(opts: Opts) -> Result<()> {
    let client = new_client(&opts)?;
    let mut headers = opts.headers.clone();
    headers.append("acl", HeaderValue::from_str("bucket-owner-full-control")?);
    let mut ext = Extensions::new();
    ext.insert(RequestBodyType::Streaming);
    let resp = client
        .post(opts.url)
        .headers(headers)
        .body("foobar")
        .with_extension(ext)
        .send()
        .await?;
    let code = resp.status();
    if !code.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|err| format!("<<could not get resp body: {err}>>"));
        bail!("request failed with {code}: {body}");
    }
    Ok(())
}

#[tokio::test]
async fn test_mw_ext() {
    let server = MockServer::start().await;
    Mock::given(matchers::method(Method::POST))
        .and(matchers::path("/upload"))
        .and(matchers::header(
            "signed-url-header-name",
            "signed-url-header-value",
        ))
        .and(matchers::header("acl", "bucket-owner-full-control"))
        .and(matchers::body_string("supervisor log content"))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .mount(&server)
        .await;
    let url: Url = server.uri().parse().unwrap();
    let url = url.join("upload").unwrap();
    let mut headers = HeaderMap::default();
    headers.append(
        "signed-url-header-name",
        HeaderValue::from_str("signed-url-header-value").unwrap(),
    );
    let opts = Opts { url, headers };
    make_request(opts).await.unwrap();
}

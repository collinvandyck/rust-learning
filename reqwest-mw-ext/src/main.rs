#![allow(unused)]

use core::panic;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use http::Extensions;
use reqwest::{Method, Request, Response, StatusCode, Url};
use reqwest_middleware::{ClientWithMiddleware, Middleware, Next};
use reqwest_retry::{policies::ExponentialBackoff, Jitter, RetryTransientMiddleware};
use tracing_test::traced_test;
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

struct Client {
    delegate: ClientWithMiddleware,
    body_type: Arc<Mutex<Option<RequestBodyType>>>,
}

fn new_client(opts: &Opts) -> Result<Client> {
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
            .jitter(Jitter::Bounded)
            .retry_bounds(Duration::from_secs(1), Duration::from_secs(5))
            .build_with_max_retries(1),
    );
    let body_type: Arc<Mutex<Option<RequestBodyType>>> = Arc::default();
    let delegate: ClientWithMiddleware = reqwest_middleware::ClientBuilder::new(delegate)
        .with(StreamingMW {
            body_type: body_type.clone(),
        })
        .with(retry_mw)
        .build();
    let client = Client {
        delegate,
        body_type,
    };
    Ok(client)
}

struct StreamingMW {
    body_type: Arc<Mutex<Option<RequestBodyType>>>,
}

#[async_trait]
impl Middleware for StreamingMW {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> std::result::Result<Response, reqwest_middleware::Error> {
        let body_type = extensions.get::<RequestBodyType>().copied();
        *self.body_type.lock().unwrap() = body_type;
        next.run(req, extensions).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestBodyType {
    Streaming,
    Static,
}

async fn make_request(opts: Opts) -> Result<Option<RequestBodyType>> {
    tracing::info!("making req");
    let client = new_client(&opts)?;
    let mut headers = opts.headers.clone();
    headers.append("acl", HeaderValue::from_str("bucket-owner-full-control")?);
    let mut ext = Extensions::new();
    ext.insert(RequestBodyType::Streaming);
    let resp = client
        .delegate
        .post(opts.url)
        .headers(headers)
        .body("supervisor log content")
        .with_extension(ext)
        .send()
        .await?;
    let code = resp.status();
    tracing::info!("got code: {code}");
    if !code.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|err| format!("<<could not get resp body: {err}>>"));
        bail!("request failed with {code}: {body}");
    }
    let body_type = client.body_type.lock().unwrap().clone();
    Ok(body_type)
}

#[tokio::test]
#[traced_test]
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
    let body_type = make_request(opts).await.unwrap();
    assert_eq!(body_type, Some(RequestBodyType::Streaming));
}

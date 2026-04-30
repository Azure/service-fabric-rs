// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! End-to-end tests for [`ResolveStatusMiddleware`] using a mock
//! inner `Service` that returns scripted bodies with trailers.

#![cfg(feature = "tonic")]

use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::future::BoxFuture;
use http::{HeaderMap, HeaderName, HeaderValue};
use http_body::{Body as _, Frame};
use tonic::body::Body;
use tower::Service;

use mssf_util::tonic::ResolveStatusMiddleware;

/// Counts how many times `()` is called.
#[derive(Clone, Default)]
struct RebuildCounter(Arc<AtomicUsize>);
impl RebuildCounter {
    fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
    fn fire(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Body that yields one trailers frame (or no trailer at all).
struct ScriptedBody {
    trailers: Option<HeaderMap>,
    done: bool,
}

impl ScriptedBody {
    fn with_trailer(name: &HeaderName, value: &str) -> Self {
        let mut map = HeaderMap::new();
        map.insert(name.clone(), HeaderValue::from_str(value).unwrap());
        Self {
            trailers: Some(map),
            done: false,
        }
    }
    fn no_trailer() -> Self {
        Self {
            trailers: None,
            done: false,
        }
    }
}

impl http_body::Body for ScriptedBody {
    type Data = Bytes;
    type Error = tonic::Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        if self.done {
            return Poll::Ready(None);
        }
        self.done = true;
        match self.trailers.take() {
            Some(map) => Poll::Ready(Some(Ok(Frame::trailers(map)))),
            None => Poll::Ready(None),
        }
    }
}

/// `Service` that returns scripted responses in order.
#[derive(Clone)]
struct ScriptedService {
    queue: Arc<std::sync::Mutex<std::collections::VecDeque<Body>>>,
}

impl ScriptedService {
    fn new<I: IntoIterator<Item = Body>>(items: I) -> Self {
        Self {
            queue: Arc::new(std::sync::Mutex::new(items.into_iter().collect())),
        }
    }
}

impl Service<http::Request<Body>> for ScriptedService {
    type Response = http::Response<Body>;
    type Error = tonic::transport::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: http::Request<Body>) -> Self::Future {
        let body = self
            .queue
            .lock()
            .unwrap()
            .pop_front()
            .expect("script exhausted");
        Box::pin(async move { Ok(http::Response::builder().body(body).unwrap()) })
    }
}

/// Drive the middleware once and return after the body is fully
/// consumed (so the trailer observer has fired).
async fn dispatch(svc: &mut ResolveStatusMiddleware<ScriptedService>) {
    use std::future::poll_fn;
    let req = http::Request::builder().body(Body::empty()).unwrap();
    let resp = svc.call(req).await.expect("ok");
    // Drain the body so the wrapper observes trailers / EOS.
    let mut body = resp.into_body();
    loop {
        let frame = poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await;
        if frame.is_none() {
            break;
        }
    }
}

fn header() -> HeaderName {
    HeaderName::from_static("mssf-status")
}

fn build(
    svc: ScriptedService,
    counter: RebuildCounter,
) -> ResolveStatusMiddleware<ScriptedService> {
    ResolveStatusMiddleware::new(svc, header(), move || counter.fire())
}

#[tokio::test]
async fn trailer_on_empty_body_triggers_one_rebuild() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![Body::new(ScriptedBody::with_trailer(
        &header(),
        "not-primary",
    ))]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    assert_eq!(counter.count(), 1);
}

#[tokio::test]
async fn same_value_dedups_to_one_rebuild() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![
        Body::new(ScriptedBody::with_trailer(&header(), "v")),
        Body::new(ScriptedBody::with_trailer(&header(), "v")),
        Body::new(ScriptedBody::with_trailer(&header(), "v")),
    ]);
    let mut mw = build(svc, counter.clone());
    for _ in 0..3 {
        dispatch(&mut mw).await;
    }
    assert_eq!(counter.count(), 1);
}

#[tokio::test]
async fn distinct_values_each_rebuild() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![
        Body::new(ScriptedBody::with_trailer(&header(), "a")),
        Body::new(ScriptedBody::with_trailer(&header(), "b")),
    ]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    assert_eq!(counter.count(), 2);
}

#[tokio::test]
async fn no_trailer_resets_dedup_state() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![
        Body::new(ScriptedBody::with_trailer(&header(), "v")),
        Body::new(ScriptedBody::no_trailer()),
        Body::new(ScriptedBody::with_trailer(&header(), "v")),
    ]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    // First v -> rebuild #1; no-trailer -> reset; v again -> rebuild #2.
    assert_eq!(counter.count(), 2);
}

#[tokio::test]
async fn empty_value_always_rebuilds_without_poisoning_last_seen() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![
        Body::new(ScriptedBody::with_trailer(&header(), "v")), // rebuild #1
        Body::new(ScriptedBody::with_trailer(&header(), "")),  // rebuild #2
        Body::new(ScriptedBody::with_trailer(&header(), "v")), // dedup vs prev v -> no rebuild
    ]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    assert_eq!(counter.count(), 2);
}

#[tokio::test]
async fn empty_value_twice_rebuilds_twice() {
    let counter = RebuildCounter::default();
    let svc = ScriptedService::new(vec![
        Body::new(ScriptedBody::with_trailer(&header(), "")),
        Body::new(ScriptedBody::with_trailer(&header(), "")),
    ]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    dispatch(&mut mw).await;
    assert_eq!(counter.count(), 2);
}

#[tokio::test]
async fn unrelated_header_does_not_trigger_rebuild() {
    let counter = RebuildCounter::default();
    let other = HeaderName::from_static("x-other");
    let svc = ScriptedService::new(vec![Body::new(ScriptedBody::with_trailer(
        &other, "anything",
    ))]);
    let mut mw = build(svc, counter.clone());
    dispatch(&mut mw).await;
    assert_eq!(counter.count(), 0);
}

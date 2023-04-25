use std::{task::{Poll}, pin::Pin, process::Output, future::Future, convert::Infallible};

use axum::{RequestExt};
use hyper::Body;
use tokio::pin;
use tower::{Service, Layer};
use http::Response;
use crate::{AutoResp, Responsor};

#[derive(Clone)]
pub struct AutoResponsor {
    
}

#[derive(Clone)]
pub struct AutoResponsorMiddleware<S> {
    inner: S,
}

impl AutoResponsor {
    pub fn new() -> Self {
        Self {}
    }
}

impl <S> Layer<S> for AutoResponsor {
    type Service = AutoResponsorMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AutoResponsorMiddleware { inner }
    }
}

impl Service<http::Request<Body>> for AutoResponsor {
    type Response = http::Response<Body>;

    type Error = Infallible;

    type Future = Pin<Box<dyn Send + Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        Box::pin(async {
            Ok(Response::default())
        })
    }
}
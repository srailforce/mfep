use std::convert::Infallible;
use std::net::SocketAddr;

use axum::Router;
use http::{Request, Response, HeaderName, HeaderValue};
use hyper::{Body, server::conn::AddrStream};
use hyper::service::make_service_fn;
use mfep::{load_config, service::responsor::AutoResponsor};
use axum::{Server, ServiceExt};
use tower::{Service, service_fn, ServiceBuilder};
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = load_config("config.yaml")?;

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            let svc = AutoResponsor::new();
            let stack = ServiceBuilder::new()
            .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*")))
            .service(svc);
            Ok::<_, Infallible>(stack)
        }
    });
    
    let s: SocketAddr = "0.0.0.0:8080".parse()?;
    let server = 
        Server::bind(&s)
        .serve(make_svc);

    server.await?;

    Ok(())
}

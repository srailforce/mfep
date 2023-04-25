use std::convert::Infallible;
use std::net::SocketAddr;

use http::{HeaderValue};
use hyper::{server::conn::AddrStream};
use hyper::service::make_service_fn;
use mfep::{load_config, service::responsor::AutoResponsor};
use axum::{Server};
use tower::{ServiceBuilder};
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = load_config("config.yaml")?;

    let make_svc = make_service_fn(|_: &AddrStream| {
        async move {
            let svc = AutoResponsor::new();
            let stack = ServiceBuilder::new()
            .layer(SetResponseHeaderLayer::overriding(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*")))
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

use anyhow::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, E> {
    Ok(Response::new(Body::from("Hello World")))
}

#[allow(clippy::uninlined_format_args)] // Cannot use format! with async functions vars
#[tokio::main]
async fn main() {
    // connection builder
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    // client builder
    let client: Client<_, hyper::Body> = Client::builder().build(https);

    // Define Socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    let make_service = make_service_fn(|_conn| async { Ok::<_>(service_fn(handle)) });

    // let body = res.body();

    Server::bind(&addr)
        .serve(make_service)
        .await
        .context("Running Server")
        .unwrap();

    // println!("body: {:?}", body);
}

struct CachedData {
    body: Response<Body>,
    timestamp: i64,
}
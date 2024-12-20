use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

// Define the service function
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // Define the address to bind the server to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a make_service_fn
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // Bind the server to the address and serve the make_svc
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Respond with "Hello, World!"
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // The address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create a service to handle requests
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Create and bind the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

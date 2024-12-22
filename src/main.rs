use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

// Define the service function
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // HTML content with CSS styling
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Styled Page</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    text-align: center;
                    background-color: #f0f0f0;
                    margin: 0;
                    padding: 0;
                }
                h1 {
                    color: #333;
                    margin-top: 20px;
                }
                p {
                    color: #666;
                }
            </style>
        </head>
        <body>
            <h1>Hypasus (Rust edition)</h1>
            <p>This is Basey's first Rust server.</p>
        </body>

        <body>
            <h1>Here is an Image</h1>
            <img src="https://imgur.com/a/KE6FvdV.png" alt="Example Image">
        </body>
        </html>
    "#;

    // Respond with the HTML content
    Ok(Response::new(Body::from(html)))
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

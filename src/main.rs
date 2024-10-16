use axum::{
    routing::{get, post},
    Router,
};
use hyper::Server;
use std::net::SocketAddr;

mod endpoints;
mod models;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new()
        .route("/", get(endpoints::hello_world))
        .route("/stringify", post(endpoints::stringify_json));

    // Address that server will bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Run the server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    routing::{get, patch},
    Router,
};
use endpoints::{create_item, delete_item, read_items, update_item};
use models::Item;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

mod endpoints;
mod models;
#[cfg(test)]
mod tests;

type Db = Arc<RwLock<HashMap<Uuid, Item>>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    let db = Db::default();

    let app = app(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn app(db: Db) -> Router {
    Router::new()
        .route("/", get(endpoints::hello_world))
        .route("/items", get(read_items).post(create_item))
        .route("/items/{id}", patch(update_item).delete(delete_item))
        .with_state(db)
}

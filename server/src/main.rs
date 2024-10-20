use axum::{
    routing::{get, patch},
    Router,
};
use endpoints::{create_item, delete_item, read_item, read_items, read_items_html, update_item};
use models::Item;
use serde_json::{from_reader, to_writer};
use std::error::Error;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    sync::{Arc, RwLock},
};
use tokio::signal;
use uuid::Uuid;

mod endpoints;
mod models;
#[cfg(test)]
mod tests;

type Db = Arc<RwLock<HashMap<Uuid, Item>>>;

#[tokio::main]
async fn main() {
    let db = match load_state_from_json() {
        Ok(db) => db,
        Err(e) => {
            println!("Warning: Failed to load state from JSON: {}", e);
            Db::default()
        }
    };

    // Save state to file on Ctrl+C signal
    let db_clone = db.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        save_state_as_json(&db_clone);
        std::process::exit(0);
    });

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
        .route("/items", get(read_items_html).post(create_item))
        .route(
            "/items/{id}",
            patch(update_item).delete(delete_item).get(read_item),
        )
        .with_state(db)
}

fn save_state_as_json(db: &Db) {
    let db_reader = db.read().unwrap();
    let items: Vec<Item> = db_reader.values().cloned().collect();
    let file = File::create("state.json").expect("Unable to create file");
    to_writer(file, &items).expect("Unable to write data");
    println!("Saved state to json file: state.json")
}

fn load_state_from_json() -> Result<Db, Box<dyn Error>> {
    let file = File::open("state.json")?;
    let reader = BufReader::new(file);
    let items: Vec<Item> = from_reader(reader)?;
    let db: HashMap<Uuid, Item> = items.into_iter().map(|item| (item.id, item)).collect();
    Ok(Arc::new(RwLock::new(db)))
}
